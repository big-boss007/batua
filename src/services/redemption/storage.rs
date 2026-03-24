use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::ledger::types::BucketType;

use super::types::{RedemptionRequest, RedemptionState, WalletPolicy};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_redemption(
    pool: &PgPool,
    merchant_id: Uuid,
    wallet_id: Uuid,
    order_id: &str,
    order_amount: f64,
    payment_method: Option<&str>,
    requested_amount: f64,
) -> Result<RedemptionRequest, AppError> {
    let row: RedemptionRequest = sqlx::query_as(
        r#"
        INSERT INTO redemption_requests (
            merchant_id, wallet_id, order_id, order_amount,
            payment_method, requested_amount, state
        )
        VALUES ($1, $2, $3, $4, $5, $6, 'initiated')
        RETURNING *
        "#,
    )
    .bind(merchant_id)
    .bind(wallet_id)
    .bind(order_id)
    .bind(order_amount)
    .bind(payment_method)
    .bind(requested_amount)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_redemption_state(
    pool: &PgPool,
    id: Uuid,
    state: &RedemptionState,
    eligible_amount: Option<f64>,
    applied_amount: Option<f64>,
    debit_entry_id: Option<Uuid>,
    compensation_entry_id: Option<Uuid>,
    shopify_discount_id: Option<&str>,
    rejection_reason: Option<&str>,
) -> Result<RedemptionRequest, AppError> {
    let row: RedemptionRequest = sqlx::query_as(
        r#"
        UPDATE redemption_requests
        SET state = $2,
            eligible_amount = COALESCE($3, eligible_amount),
            applied_amount = COALESCE($4, applied_amount),
            debit_entry_id = COALESCE($5, debit_entry_id),
            compensation_entry_id = COALESCE($6, compensation_entry_id),
            shopify_discount_id = COALESCE($7, shopify_discount_id),
            rejection_reason = COALESCE($8, rejection_reason),
            updated_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(state)
    .bind(eligible_amount)
    .bind(applied_amount)
    .bind(debit_entry_id)
    .bind(compensation_entry_id)
    .bind(shopify_discount_id)
    .bind(rejection_reason)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("redemption request {id} not found")))?;

    Ok(row)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_redemption(pool: &PgPool, id: Uuid) -> Result<RedemptionRequest, AppError> {
    let row: RedemptionRequest = sqlx::query_as(
        "SELECT * FROM redemption_requests WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("redemption request {id} not found")))?;

    Ok(row)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_wallet_policy(
    pool: &PgPool,
    merchant_id: Uuid,
    bucket_type: &BucketType,
) -> Result<Option<WalletPolicy>, AppError> {
    let row: Option<WalletPolicy> = sqlx::query_as(
        r#"
        SELECT id, merchant_id, bucket_type, min_redemption, step_size,
               max_per_order_pct, max_per_order_fixed, stackable_with_discounts,
               default_expiry_days, excluded_payment_methods,
               is_active, created_at, updated_at
        FROM wallet_policies
        WHERE merchant_id = $1 AND bucket_type = $2 AND is_active = true
        "#,
    )
    .bind(merchant_id)
    .bind(bucket_type)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_wallet_policies(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<WalletPolicy>, AppError> {
    let rows: Vec<WalletPolicy> = sqlx::query_as(
        r#"
        SELECT id, merchant_id, bucket_type, min_redemption, step_size,
               max_per_order_pct, max_per_order_fixed, stackable_with_discounts,
               default_expiry_days, excluded_payment_methods,
               is_active, created_at, updated_at
        FROM wallet_policies
        WHERE merchant_id = $1 AND is_active = true
        ORDER BY bucket_type
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
