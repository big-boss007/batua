use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::ledger::types::ActorType;

use super::types::{GiftCard, GiftCardStats};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_gift_card(
    pool: &PgPool,
    merchant_id: Uuid,
    wallet_id: Uuid,
    code: &str,
    initial_amount: f64,
    issued_by: &ActorType,
    issued_by_id: Option<&str>,
    payment_reference: Option<&str>,
    batch_id: Option<Uuid>,
    batch_position: Option<i32>,
    expires_at: Option<DateTime<Utc>>,
) -> Result<GiftCard, AppError> {
    let gift_card = sqlx::query_as::<_, GiftCard>(
        r#"
        INSERT INTO gift_cards (
            merchant_id, wallet_id, code,
            initial_amount, current_amount,
            issued_by, issued_by_id, payment_reference,
            batch_id, batch_position, expires_at
        )
        VALUES ($1, $2, $3, $4, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(merchant_id)
    .bind(wallet_id)
    .bind(code)
    .bind(initial_amount)
    .bind(issued_by)
    .bind(issued_by_id)
    .bind(payment_reference)
    .bind(batch_id)
    .bind(batch_position)
    .bind(expires_at)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!("gift card with code {code} already exists"))
        }
        other => AppError::Database(other),
    })?;

    Ok(gift_card)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_gift_card_by_code(pool: &PgPool, code: &str) -> Result<GiftCard, AppError> {
    let gift_card = sqlx::query_as::<_, GiftCard>(
        "SELECT * FROM gift_cards WHERE code = $1",
    )
    .bind(code)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("gift card with code {code} not found")))?;

    Ok(gift_card)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_gift_card(pool: &PgPool, id: Uuid) -> Result<GiftCard, AppError> {
    let gift_card = sqlx::query_as::<_, GiftCard>(
        "SELECT * FROM gift_cards WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("gift card {id} not found")))?;

    Ok(gift_card)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_gift_card_amount(
    pool: &PgPool,
    id: Uuid,
    new_amount: f64,
) -> Result<GiftCard, AppError> {
    let gift_card = sqlx::query_as::<_, GiftCard>(
        r#"
        UPDATE gift_cards
        SET current_amount = $2, updated_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(new_amount)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("gift card {id} not found")))?;

    Ok(gift_card)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn claim_gift_card(
    pool: &PgPool,
    id: Uuid,
    claimed_by_wallet_id: Uuid,
) -> Result<GiftCard, AppError> {
    let gift_card = sqlx::query_as::<_, GiftCard>(
        r#"
        UPDATE gift_cards
        SET is_claimed = true,
            claimed_by_wallet_id = $2,
            claimed_at = now(),
            updated_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(claimed_by_wallet_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("gift card {id} not found")))?;

    Ok(gift_card)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_gift_cards(
    pool: &PgPool,
    merchant_id: Uuid,
    page: i32,
    limit: i32,
) -> Result<Vec<GiftCard>, AppError> {
    let offset = (page - 1) * limit;

    let cards = sqlx::query_as::<_, GiftCard>(
        r#"
        SELECT * FROM gift_cards
        WHERE merchant_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(merchant_id)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    Ok(cards)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_batch_gift_cards(
    pool: &PgPool,
    batch_id: Uuid,
) -> Result<Vec<GiftCard>, AppError> {
    let cards = sqlx::query_as::<_, GiftCard>(
        r#"
        SELECT * FROM gift_cards
        WHERE batch_id = $1
        ORDER BY batch_position ASC
        "#,
    )
    .bind(batch_id)
    .fetch_all(pool)
    .await?;

    Ok(cards)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_gift_card_by_batch_position(
    pool: &PgPool,
    batch_id: Uuid,
    batch_position: i32,
) -> Result<Option<GiftCard>, AppError> {
    let card = sqlx::query_as::<_, GiftCard>(
        "SELECT * FROM gift_cards WHERE batch_id = $1 AND batch_position = $2",
    )
    .bind(batch_id)
    .bind(batch_position)
    .fetch_optional(pool)
    .await?;

    Ok(card)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_gift_card_stats(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<GiftCardStats, AppError> {
    let row: (i64, f64, f64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COUNT(*)::int8,
            COALESCE(SUM(current_amount), 0)::float8,
            COALESCE(SUM(initial_amount - current_amount), 0)::float8,
            COUNT(*) FILTER (WHERE is_active = false AND expires_at IS NOT NULL AND expires_at < now())::int8,
            COUNT(*) FILTER (WHERE is_active = true)::int8,
            COUNT(*) FILTER (WHERE is_claimed = true)::int8
        FROM gift_cards
        WHERE merchant_id = $1
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    Ok(GiftCardStats {
        total_issued: row.0,
        total_outstanding_value: row.1,
        total_redeemed_value: row.2,
        total_expired: row.3,
        total_active: row.4,
        total_claimed: row.5,
    })
}
