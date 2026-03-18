use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::types::{CodAnalytics, CodOrder};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_cod_order(
    pool: &PgPool,
    merchant_id: Uuid,
    order_id: &str,
    wallet_id: Uuid,
    ledger_entry_id: Uuid,
) -> Result<CodOrder, AppError> {
    let order: CodOrder = sqlx::query_as(
        r#"
        INSERT INTO cod_orders (merchant_id, order_id, wallet_id, ledger_entry_id, state)
        VALUES ($1, $2, $3, $4, 'pending')
        RETURNING *
        "#,
    )
    .bind(merchant_id)
    .bind(order_id)
    .bind(wallet_id)
    .bind(ledger_entry_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!(
                "COD order already exists for merchant {merchant_id} and order {order_id}"
            ))
        }
        other => AppError::Database(other),
    })?;

    Ok(order)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_cod_order_by_order_id(
    pool: &PgPool,
    merchant_id: Uuid,
    order_id: &str,
) -> Result<CodOrder, AppError> {
    let order: CodOrder = sqlx::query_as(
        r#"
        SELECT * FROM cod_orders
        WHERE merchant_id = $1 AND order_id = $2
        "#,
    )
    .bind(merchant_id)
    .bind(order_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        AppError::NotFound(format!(
            "COD order not found for merchant {merchant_id} and order {order_id}"
        ))
    })?;

    Ok(order)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_cod_state(
    pool: &PgPool,
    id: Uuid,
    state: &str,
    released_entry_id: Option<Uuid>,
    cancelled_entry_id: Option<Uuid>,
    delivery_confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<CodOrder, AppError> {
    let order: CodOrder = sqlx::query_as(
        r#"
        UPDATE cod_orders
        SET state = $2,
            released_entry_id = COALESCE($3, released_entry_id),
            cancelled_entry_id = COALESCE($4, cancelled_entry_id),
            delivery_confirmed_at = COALESCE($5, delivery_confirmed_at),
            updated_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(state)
    .bind(released_entry_id)
    .bind(cancelled_entry_id)
    .bind(delivery_confirmed_at)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("COD order {id} not found")))?;

    Ok(order)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_pending_cod_orders(
    pool: &PgPool,
    merchant_id: Uuid,
    state_filter: Option<&str>,
    page: i32,
    limit: i32,
) -> Result<Vec<CodOrder>, AppError> {
    let offset = (page - 1) * limit;

    let orders: Vec<CodOrder> = match state_filter {
        Some(state) => {
            sqlx::query_as(
                r#"
                SELECT * FROM cod_orders
                WHERE merchant_id = $1 AND state = $2
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(merchant_id)
            .bind(state)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as(
                r#"
                SELECT * FROM cod_orders
                WHERE merchant_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(merchant_id)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await?
        }
    };

    Ok(orders)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_cod_analytics(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<CodAnalytics, AppError> {
    #[derive(sqlx::FromRow)]
    struct AnalyticsRow {
        total_pending: Option<i64>,
        total_delivered: Option<i64>,
        total_rto: Option<i64>,
        pending_amount: Option<f64>,
        released_amount: Option<f64>,
        cancelled_amount: Option<f64>,
    }

    let row: AnalyticsRow = sqlx::query_as(
        r#"
        SELECT
            COALESCE(COUNT(*) FILTER (WHERE co.state = 'pending'), 0)::int8 AS total_pending,
            COALESCE(COUNT(*) FILTER (WHERE co.state = 'delivered'), 0)::int8 AS total_delivered,
            COALESCE(COUNT(*) FILTER (WHERE co.state = 'rto'), 0)::int8 AS total_rto,
            COALESCE(SUM(le.earning_unit) FILTER (WHERE co.state = 'pending'), 0)::float8 AS pending_amount,
            COALESCE(SUM(le.earning_unit) FILTER (WHERE co.state = 'delivered'), 0)::float8 AS released_amount,
            COALESCE(SUM(le.earning_unit) FILTER (WHERE co.state = 'rto'), 0)::float8 AS cancelled_amount
        FROM cod_orders co
        JOIN ledger_entries le ON le.id = co.ledger_entry_id
        WHERE co.merchant_id = $1
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    Ok(CodAnalytics {
        total_pending: row.total_pending.unwrap_or(0),
        total_delivered: row.total_delivered.unwrap_or(0),
        total_rto: row.total_rto.unwrap_or(0),
        pending_amount: row.pending_amount.unwrap_or(0.0),
        released_amount: row.released_amount.unwrap_or(0.0),
        cancelled_amount: row.cancelled_amount.unwrap_or(0.0),
    })
}
