use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, sqlx::FromRow)]
pub struct CustomerOrderStats {
    pub total_orders: i32,
    pub total_spend: f64,
    pub first_order_at: Option<DateTime<Utc>>,
    pub last_order_at: Option<DateTime<Utc>>,
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_customer_order_stats(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<Option<CustomerOrderStats>, AppError> {
    let stats = sqlx::query_as::<_, CustomerOrderStats>(
        r#"
        SELECT total_orders, total_spend::float8 AS total_spend,
               first_order_at, last_order_at
        FROM customer_order_stats
        WHERE merchant_id = $1 AND customer_id = $2
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await?;

    Ok(stats)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_order_stats(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    order_amount: f64,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO customer_order_stats (merchant_id, customer_id, total_orders, total_spend, first_order_at, last_order_at)
        VALUES ($1, $2, 1, $3, now(), now())
        ON CONFLICT (merchant_id, customer_id)
        DO UPDATE SET
            total_orders = customer_order_stats.total_orders + 1,
            total_spend = customer_order_stats.total_spend + $3,
            last_order_at = now()
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .bind(order_amount)
    .execute(pool)
    .await?;

    Ok(())
}
