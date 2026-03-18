use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::ledger::storage as ledger_storage;
use crate::services::ledger::types::{
    ActorType, BucketType, MovementType, NewLedgerEntry,
};
use crate::services::wallets::storage as wallet_storage;

use super::storage;
use super::types::{CodOrderState, CodToPrepaidRequest, CodToPrepaidResponse};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_delivery(
    pool: &PgPool,
    order_id: &str,
    merchant_id: Uuid,
) -> Result<(), AppError> {
    let cod_order = storage::get_cod_order_by_order_id(pool, merchant_id, order_id).await?;

    if cod_order.state != CodOrderState::Pending.as_str() {
        return Err(AppError::BadRequest(format!(
            "COD order {} is in state '{}', expected 'pending'",
            order_id, cod_order.state
        )));
    }

    let held_entry = ledger_storage::get_entries(
        pool,
        cod_order.wallet_id,
        crate::services::ledger::types::GetEntriesQuery {
            page: Some(1),
            limit: Some(1),
            bucket_type: Some(BucketType::CodPending),
            movement_type: Some(MovementType::Held),
        },
    )
    .await?;

    let held = held_entry.into_iter().find(|e| e.id == cod_order.ledger_entry_id).ok_or_else(|| {
        AppError::NotFound(format!(
            "held ledger entry {} not found for COD order {}",
            cod_order.ledger_entry_id, order_id
        ))
    })?;

    let out_entry = NewLedgerEntry {
        wallet_id: cod_order.wallet_id,
        bucket_type: BucketType::CodPending,
        movement_type: MovementType::Out,
        earning_unit: held.earning_unit,
        currency_equivalent: held.currency_equivalent,
        conversion_rate: held.conversion_rate,
        event_id: held.event_id,
        rule_snapshot_id: held.rule_snapshot_id,
        campaign_snapshot_id: held.campaign_snapshot_id,
        actor_type: ActorType::System,
        actor_id: Some("cod-delivery".to_string()),
        payment_reference: held.payment_reference.clone(),
        transfer_id: None,
        constraints: held.constraints.clone(),
        expires_at: None,
    };

    let in_entry = NewLedgerEntry {
        wallet_id: cod_order.wallet_id,
        bucket_type: BucketType::EarnedCredit,
        movement_type: MovementType::In,
        earning_unit: held.earning_unit,
        currency_equivalent: held.currency_equivalent,
        conversion_rate: held.conversion_rate,
        event_id: held.event_id,
        rule_snapshot_id: held.rule_snapshot_id,
        campaign_snapshot_id: held.campaign_snapshot_id,
        actor_type: ActorType::System,
        actor_id: Some("cod-delivery".to_string()),
        payment_reference: held.payment_reference.clone(),
        transfer_id: None,
        constraints: serde_json::json!({}),
        expires_at: None,
    };

    let out_key = format!("cod-delivery-out-{}-{}", merchant_id, order_id);
    let in_key = format!("cod-delivery-in-{}-{}", merchant_id, order_id);

    let (_out_result, in_result) =
        ledger_storage::create_across_movement(pool, out_entry, out_key, in_entry, in_key).await?;

    storage::update_cod_state(
        pool,
        cod_order.id,
        CodOrderState::Delivered.as_str(),
        Some(in_result.id),
        None,
        Some(chrono::Utc::now()),
    )
    .await?;

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_rto(
    pool: &PgPool,
    order_id: &str,
    merchant_id: Uuid,
) -> Result<(), AppError> {
    let cod_order = storage::get_cod_order_by_order_id(pool, merchant_id, order_id).await?;

    if cod_order.state != CodOrderState::Pending.as_str() {
        return Err(AppError::BadRequest(format!(
            "COD order {} is in state '{}', expected 'pending'",
            order_id, cod_order.state
        )));
    }

    let held_entry = ledger_storage::get_entries(
        pool,
        cod_order.wallet_id,
        crate::services::ledger::types::GetEntriesQuery {
            page: Some(1),
            limit: Some(1),
            bucket_type: Some(BucketType::CodPending),
            movement_type: Some(MovementType::Held),
        },
    )
    .await?;

    let held = held_entry.into_iter().find(|e| e.id == cod_order.ledger_entry_id).ok_or_else(|| {
        AppError::NotFound(format!(
            "held ledger entry {} not found for COD order {}",
            cod_order.ledger_entry_id, order_id
        ))
    })?;

    let cancel_entry = NewLedgerEntry {
        wallet_id: cod_order.wallet_id,
        bucket_type: BucketType::CodPending,
        movement_type: MovementType::Out,
        earning_unit: held.earning_unit,
        currency_equivalent: held.currency_equivalent,
        conversion_rate: held.conversion_rate,
        event_id: held.event_id,
        rule_snapshot_id: held.rule_snapshot_id,
        campaign_snapshot_id: held.campaign_snapshot_id,
        actor_type: ActorType::System,
        actor_id: Some("cod-rto".to_string()),
        payment_reference: held.payment_reference.clone(),
        transfer_id: None,
        constraints: held.constraints.clone(),
        expires_at: None,
    };

    let idempotency_key = format!("cod-rto-{}-{}", merchant_id, order_id);
    let cancelled_entry =
        ledger_storage::create_entry(pool, cancel_entry, idempotency_key).await?;

    storage::update_cod_state(
        pool,
        cod_order.id,
        CodOrderState::Rto.as_str(),
        None,
        Some(cancelled_entry.id),
        None,
    )
    .await?;

    Ok(())
}

pub fn calculate_cod_incentive(order_amount: f64) -> f64 {
    let percentage_incentive = order_amount * 0.02;
    let flat_incentive = 75.0;
    percentage_incentive.max(flat_incentive).min(order_amount)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_cod_to_prepaid(
    pool: &PgPool,
    req: CodToPrepaidRequest,
) -> Result<CodToPrepaidResponse, AppError> {
    if req.new_payment_method != "upi" {
        return Err(AppError::BadRequest(format!(
            "unsupported payment method: {}, only 'upi' is supported",
            req.new_payment_method
        )));
    }

    let wallet =
        wallet_storage::get_or_create_wallet(pool, req.merchant_id, req.customer_id).await?;

    let incentive_amount = calculate_cod_incentive(req.order_amount);

    let entry = NewLedgerEntry {
        wallet_id: wallet.id,
        bucket_type: BucketType::EarnedCredit,
        movement_type: MovementType::In,
        earning_unit: incentive_amount,
        currency_equivalent: incentive_amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some("cod-to-prepaid".to_string()),
        payment_reference: Some(req.order_id.clone()),
        transfer_id: None,
        constraints: serde_json::json!({}),
        expires_at: None,
    };

    let idempotency_key = format!(
        "cod-prepaid-{}-{}-{}",
        req.merchant_id, req.customer_id, req.order_id
    );
    let ledger_entry = ledger_storage::create_entry(pool, entry, idempotency_key).await?;

    Ok(CodToPrepaidResponse {
        incentive_amount,
        ledger_entry_id: ledger_entry.id,
        message: format!(
            "COD-to-prepaid incentive of {:.2} credited for order {}",
            incentive_amount, req.order_id
        ),
    })
}
