use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::types::{CreateWalletRequest, Wallet};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_wallet(pool: &PgPool, req: &CreateWalletRequest) -> Result<Wallet, AppError> {
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        INSERT INTO wallets (merchant_id, customer_id, is_bearer, bearer_code)
        VALUES ($1, $2, $3, $4)
        RETURNING id, merchant_id, customer_id, is_bearer, bearer_code, created_at
        "#,
    )
    .bind(req.merchant_id)
    .bind(req.customer_id)
    .bind(req.is_bearer)
    .bind(&req.bearer_code)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("wallet already exists for this merchant and customer".to_string())
        }
        other => AppError::Database(other),
    })?;

    Ok(wallet)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_wallet(pool: &PgPool, id: Uuid) -> Result<Wallet, AppError> {
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, merchant_id, customer_id, is_bearer, bearer_code, created_at
        FROM wallets
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("wallet {id} not found")))?;

    Ok(wallet)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_wallet_by_merchant_customer(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<Option<Wallet>, AppError> {
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, merchant_id, customer_id, is_bearer, bearer_code, created_at
        FROM wallets
        WHERE merchant_id = $1 AND customer_id = $2
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await?;

    Ok(wallet)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_or_create_wallet(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<Wallet, AppError> {
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        INSERT INTO wallets (merchant_id, customer_id, is_bearer)
        VALUES ($1, $2, false)
        ON CONFLICT (merchant_id, customer_id) DO NOTHING
        RETURNING id, merchant_id, customer_id, is_bearer, bearer_code, created_at
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await?;

    if let Some(w) = wallet {
        return Ok(w);
    }

    let existing = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, merchant_id, customer_id, is_bearer, bearer_code, created_at
        FROM wallets
        WHERE merchant_id = $1 AND customer_id = $2
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        AppError::Internal("wallet upsert failed: row not found after insert".to_string())
    })?;

    Ok(existing)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_bearer_wallet(
    pool: &PgPool,
    bearer_code: &str,
) -> Result<Option<Wallet>, AppError> {
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, merchant_id, customer_id, is_bearer, bearer_code, created_at
        FROM wallets
        WHERE bearer_code = $1 AND is_bearer = true
        "#,
    )
    .bind(bearer_code)
    .fetch_optional(pool)
    .await?;

    Ok(wallet)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_wallets_for_merchant(
    pool: &PgPool,
    merchant_id: Uuid,
    page: i32,
    limit: i32,
) -> Result<Vec<Wallet>, AppError> {
    let offset = (page - 1) * limit;

    let wallets = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, merchant_id, customer_id, is_bearer, bearer_code, created_at
        FROM wallets
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

    Ok(wallets)
}
