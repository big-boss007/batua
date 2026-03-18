use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::ledger::storage as ledger_storage;
use crate::services::ledger::types::{
    ActorType, CreditState, LedgerEntry, MovementType, NewLedgerEntry,
};
use crate::services::wallets::storage as wallet_storage;

use super::storage;
use super::types::{
    AdminDashboard, BulkCreditItemResult, BulkCreditRequest, BulkCreditResult, DisputeRequest,
    DisputeResult,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_bulk_credit(
    pool: &PgPool,
    req: &BulkCreditRequest,
) -> Result<BulkCreditResult, AppError> {
    let total_processed = req.customer_ids.len() as i32;
    let mut total_succeeded: i32 = 0;
    let mut total_failed: i32 = 0;
    let mut results = Vec::with_capacity(req.customer_ids.len());

    let bucket_type = parse_bucket_type(&req.bucket_type)?;

    for customer_id in &req.customer_ids {
        match process_single_credit(pool, req, *customer_id, bucket_type.clone()).await {
            Ok(entry_id) => {
                total_succeeded += 1;
                results.push(BulkCreditItemResult {
                    customer_id: *customer_id,
                    success: true,
                    ledger_entry_id: Some(entry_id),
                    error: None,
                });
            }
            Err(e) => {
                total_failed += 1;
                results.push(BulkCreditItemResult {
                    customer_id: *customer_id,
                    success: false,
                    ledger_entry_id: None,
                    error: Some(e.to_string()),
                });
            }
        }
    }

    Ok(BulkCreditResult {
        total_processed,
        total_succeeded,
        total_failed,
        results,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
async fn process_single_credit(
    pool: &PgPool,
    req: &BulkCreditRequest,
    customer_id: Uuid,
    bucket_type: crate::services::ledger::types::BucketType,
) -> Result<Uuid, AppError> {
    let wallet =
        wallet_storage::get_or_create_wallet(pool, req.merchant_id, customer_id).await?;

    let idempotency_key = format!(
        "bulk-credit-{}-{}-{}",
        req.merchant_id, customer_id, req.reason
    );

    let new_entry = NewLedgerEntry {
        wallet_id: wallet.id,
        bucket_type,
        movement_type: MovementType::In,
        earning_unit: req.amount,
        currency_equivalent: req.amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::Human,
        actor_id: Some(req.actor_id.clone()),
        payment_reference: None,
        transfer_id: None,
        constraints: serde_json::json!({ "reason": req.reason }),
        expires_at: None,
    };

    let entry = ledger_storage::create_entry(pool, new_entry, idempotency_key).await?;

    Ok(entry.id)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_dispute(
    pool: &PgPool,
    req: &DisputeRequest,
) -> Result<DisputeResult, AppError> {
    let original_entry: LedgerEntry = sqlx::query_as(
        r#"
        SELECT * FROM ledger_entries WHERE id = $1
        "#,
    )
    .bind(req.ledger_entry_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        AppError::NotFound(format!("ledger entry {} not found", req.ledger_entry_id))
    })?;

    if original_entry.state == CreditState::Reversed {
        return Err(AppError::Conflict(
            "ledger entry has already been reversed".to_string(),
        ));
    }

    let idempotency_key = format!("dispute-reversal-{}", req.ledger_entry_id);

    let reversal_entry = NewLedgerEntry {
        wallet_id: original_entry.wallet_id,
        bucket_type: original_entry.bucket_type,
        movement_type: MovementType::Out,
        earning_unit: original_entry.earning_unit,
        currency_equivalent: original_entry.currency_equivalent,
        conversion_rate: original_entry.conversion_rate,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::Human,
        actor_id: Some(req.actor_id.clone()),
        payment_reference: None,
        transfer_id: None,
        constraints: serde_json::json!({
            "dispute_reason": req.reason,
            "original_entry_id": req.ledger_entry_id,
        }),
        expires_at: None,
    };

    let reversal = ledger_storage::create_entry(pool, reversal_entry, idempotency_key).await?;

    sqlx::query("UPDATE ledger_entries SET state = 'reversed' WHERE id = $1")
        .bind(req.ledger_entry_id)
        .execute(pool)
        .await?;

    Ok(DisputeResult {
        reversal_entry_id: reversal.id,
        original_amount: original_entry.earning_unit,
        reversed: true,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_system_dashboard(pool: &PgPool) -> Result<AdminDashboard, AppError> {
    storage::get_dashboard_stats(pool).await
}

fn parse_bucket_type(
    s: &str,
) -> Result<crate::services::ledger::types::BucketType, AppError> {
    match s {
        "earned_credit" => Ok(crate::services::ledger::types::BucketType::EarnedCredit),
        "cod_pending" => Ok(crate::services::ledger::types::BucketType::CodPending),
        "gift_card" => Ok(crate::services::ledger::types::BucketType::GiftCard),
        "customer_funded" => Ok(crate::services::ledger::types::BucketType::CustomerFunded),
        "referral_reward" => Ok(crate::services::ledger::types::BucketType::ReferralReward),
        "goodwill_credit" => Ok(crate::services::ledger::types::BucketType::GoodwillCredit),
        "membership_benefit" => {
            Ok(crate::services::ledger::types::BucketType::MembershipBenefit)
        }
        "refund_credit" => Ok(crate::services::ledger::types::BucketType::RefundCredit),
        other => Err(AppError::BadRequest(format!(
            "invalid bucket_type: '{other}'"
        ))),
    }
}
