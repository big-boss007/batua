use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::ledger::storage as ledger_storage;
use crate::services::ledger::types::{
    ActorType, BucketType, CreditState, LedgerEntry, MovementType, NewLedgerEntry,
};
use crate::services::wallets::storage as wallet_storage;

use super::storage;
use super::types::{
    AdminDashboard, AdminDebitRequest, AdminDebitResult, AdminExpireRequest, AdminExpireResult,
    BulkCreditItemResult, BulkCreditRequest, BulkCreditResult, CoalitionTransferRequest,
    CoalitionTransferResult, DisputeRequest, DisputeResult,
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

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn transfer_coalition_credits(
    pool: &PgPool,
    req: &CoalitionTransferRequest,
) -> Result<CoalitionTransferResult, AppError> {
    let (coalition, from_member, to_member) = storage::get_coalition_for_merchants(
        pool,
        req.from_merchant_id,
        req.to_merchant_id,
    )
    .await?
    .ok_or_else(|| {
        AppError::BadRequest("merchants are not in the same coalition".to_string())
    })?;

    let from_wallet = wallet_storage::get_wallet_by_merchant_customer(
        pool,
        req.from_merchant_id,
        req.customer_id,
    )
    .await?
    .ok_or_else(|| {
        AppError::NotFound(format!(
            "customer {} has no wallet at merchant {}",
            req.customer_id, req.from_merchant_id
        ))
    })?;

    let balance = ledger_storage::get_balance(pool, from_wallet.id).await?;
    if balance.spendable_balance < req.amount {
        return Err(AppError::BadRequest(format!(
            "insufficient balance: spendable {}, requested {}",
            balance.spendable_balance, req.amount
        )));
    }

    let to_wallet = wallet_storage::get_wallet_by_merchant_customer(
        pool,
        req.to_merchant_id,
        req.customer_id,
    )
    .await?
    .ok_or_else(|| {
        AppError::NotFound(format!(
            "customer {} has no wallet at merchant {}",
            req.customer_id, req.to_merchant_id
        ))
    })?;

    let conversion_rate = to_member.conversion_rate / from_member.conversion_rate;
    let converted_amount = req.amount * conversion_rate;

    let idempotency_base = format!(
        "coalition-{}-{}-{}-{}-{}",
        coalition.id, req.customer_id, req.from_merchant_id, req.to_merchant_id, req.amount
    );

    let out_entry = NewLedgerEntry {
        wallet_id: from_wallet.id,
        bucket_type: BucketType::EarnedCredit,
        movement_type: MovementType::Out,
        earning_unit: req.amount,
        currency_equivalent: req.amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some(format!("coalition:{}", coalition.id)),
        payment_reference: None,
        transfer_id: None,
        constraints: serde_json::json!({
            "coalition_id": coalition.id,
            "coalition_transfer": true,
            "to_merchant_id": req.to_merchant_id,
        }),
        expires_at: None,
    };

    let in_entry = NewLedgerEntry {
        wallet_id: to_wallet.id,
        bucket_type: BucketType::EarnedCredit,
        movement_type: MovementType::In,
        earning_unit: converted_amount,
        currency_equivalent: converted_amount,
        conversion_rate,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some(format!("coalition:{}", coalition.id)),
        payment_reference: None,
        transfer_id: None,
        constraints: serde_json::json!({
            "coalition_id": coalition.id,
            "coalition_transfer": true,
            "from_merchant_id": req.from_merchant_id,
        }),
        expires_at: None,
    };

    let (out_result, _in_result) = ledger_storage::create_across_movement(
        pool,
        out_entry,
        format!("{idempotency_base}-out"),
        in_entry,
        format!("{idempotency_base}-in"),
    )
    .await?;

    let transfer_id = out_result
        .transfer_id
        .unwrap_or_else(Uuid::new_v4);

    storage::record_coalition_transfer(
        pool,
        coalition.id,
        req.customer_id,
        req.from_merchant_id,
        req.to_merchant_id,
        from_wallet.id,
        to_wallet.id,
        req.amount,
        converted_amount,
        conversion_rate,
        transfer_id,
    )
    .await?;

    let from_balance = ledger_storage::get_balance(pool, from_wallet.id).await?;
    let to_balance = ledger_storage::get_balance(pool, to_wallet.id).await?;

    Ok(CoalitionTransferResult {
        transfer_id,
        from_amount: req.amount,
        to_amount: converted_amount,
        conversion_rate,
        from_balance_after: from_balance.spendable_balance,
        to_balance_after: to_balance.spendable_balance,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_debit(
    pool: &PgPool,
    req: &AdminDebitRequest,
) -> Result<AdminDebitResult, AppError> {
    let bucket_type = parse_bucket_type(&req.bucket_type)?;

    let wallet =
        wallet_storage::get_wallet_by_merchant_customer(pool, req.merchant_id, req.customer_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("customer {} has no wallet", req.customer_id))
            })?;

    let balance = ledger_storage::get_balance(pool, wallet.id).await?;
    let bucket_balance = balance
        .buckets
        .iter()
        .find(|b| b.bucket_type == bucket_type)
        .map(|b| b.spendable)
        .unwrap_or(0.0);

    if req.amount > bucket_balance {
        return Err(AppError::BadRequest(format!(
            "insufficient balance: {} has {}, requested {}",
            req.bucket_type, bucket_balance, req.amount
        )));
    }

    let idempotency_key = format!(
        "admin-debit-{}-{}-{}-{}",
        req.merchant_id,
        req.customer_id,
        req.bucket_type,
        chrono::Utc::now().timestamp_millis()
    );

    let new_entry = NewLedgerEntry {
        wallet_id: wallet.id,
        bucket_type,
        movement_type: MovementType::Out,
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
        constraints: serde_json::json!({
            "reason_category": req.reason_category,
            "reason_text": req.reason_text,
            "reference": req.reference,
            "action": "admin_debit",
        }),
        expires_at: None,
    };

    let entry = ledger_storage::create_entry(pool, new_entry, idempotency_key).await?;

    let new_balance = ledger_storage::get_balance(pool, wallet.id).await?;
    let new_bucket_bal = new_balance
        .buckets
        .iter()
        .find(|b| format!("{:?}", b.bucket_type) == format!("{:?}", entry.bucket_type))
        .map(|b| b.spendable)
        .unwrap_or(0.0);

    Ok(AdminDebitResult {
        success: true,
        ledger_entry_id: Some(entry.id),
        amount_removed: req.amount,
        new_bucket_balance: new_bucket_bal,
        new_total_balance: new_balance.spendable_balance,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_force_expire(
    pool: &PgPool,
    req: &AdminExpireRequest,
) -> Result<AdminExpireResult, AppError> {
    let wallet =
        wallet_storage::get_wallet_by_merchant_customer(pool, req.merchant_id, req.customer_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("customer {} has no wallet", req.customer_id))
            })?;

    let mut total_entries_expired: i32 = 0;
    let mut total_amount_expired: f64 = 0.0;

    for bucket_type_str in &req.bucket_types {
        let bucket_type = parse_bucket_type(bucket_type_str)?;

        let expired: Vec<LedgerEntry> = sqlx::query_as(
            r#"
            UPDATE ledger_entries
            SET state = 'expired', expires_at = now()
            WHERE wallet_id = $1
                AND bucket_type = $2
                AND state = 'active'
                AND movement_type = 'in'
            RETURNING *
            "#,
        )
        .bind(wallet.id)
        .bind(&bucket_type)
        .fetch_all(pool)
        .await?;

        let count = expired.len() as i32;
        let amount: f64 = expired.iter().map(|e| e.earning_unit).sum();
        total_entries_expired += count;
        total_amount_expired += amount;
    }

    let new_balance = ledger_storage::get_balance(pool, wallet.id).await?;

    Ok(AdminExpireResult {
        success: true,
        entries_expired: total_entries_expired,
        amount_expired: total_amount_expired,
        new_total_balance: new_balance.spendable_balance,
    })
}

fn parse_bucket_type(
    s: &str,
) -> Result<crate::services::ledger::types::BucketType, AppError> {
    match s {
        "earned_credit" | "EarnedCredit" => {
            Ok(crate::services::ledger::types::BucketType::EarnedCredit)
        }
        "cod_pending" | "CodPending" => {
            Ok(crate::services::ledger::types::BucketType::CodPending)
        }
        "gift_card" | "GiftCard" => Ok(crate::services::ledger::types::BucketType::GiftCard),
        "customer_funded" | "CustomerFunded" => {
            Ok(crate::services::ledger::types::BucketType::CustomerFunded)
        }
        "referral_reward" | "ReferralReward" => {
            Ok(crate::services::ledger::types::BucketType::ReferralReward)
        }
        "goodwill_credit" | "GoodwillCredit" => {
            Ok(crate::services::ledger::types::BucketType::GoodwillCredit)
        }
        "membership_benefit" | "MembershipBenefit" => {
            Ok(crate::services::ledger::types::BucketType::MembershipBenefit)
        }
        "refund_credit" | "RefundCredit" => {
            Ok(crate::services::ledger::types::BucketType::RefundCredit)
        }
        other => Err(AppError::BadRequest(format!(
            "invalid bucket_type: '{other}'"
        ))),
    }
}
