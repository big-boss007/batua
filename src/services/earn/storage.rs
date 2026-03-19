use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::types::{AchievedMilestone, CreateMilestoneRequest, MilestoneConfig};

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

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_milestone_config(
    pool: &PgPool,
    req: &CreateMilestoneRequest,
) -> Result<MilestoneConfig, AppError> {
    let config = sqlx::query_as::<_, MilestoneConfig>(
        r#"
        INSERT INTO milestone_configs (merchant_id, name, milestone_type, threshold, reward_amount)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(req.merchant_id)
    .bind(&req.name)
    .bind(&req.milestone_type)
    .bind(req.threshold)
    .bind(req.reward_amount)
    .fetch_one(pool)
    .await?;

    Ok(config)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_active_milestones(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<MilestoneConfig>, AppError> {
    let milestones = sqlx::query_as::<_, MilestoneConfig>(
        r#"
        SELECT id, merchant_id, name, milestone_type, threshold, reward_amount, is_active, created_at
        FROM milestone_configs
        WHERE merchant_id = $1 AND is_active = true
        ORDER BY threshold ASC
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(milestones)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn has_achieved_milestone(
    pool: &PgPool,
    customer_id: Uuid,
    milestone_id: Uuid,
) -> Result<bool, AppError> {
    let row: Option<(i32,)> = sqlx::query_as(
        r#"
        SELECT 1 AS one
        FROM milestone_achievements
        WHERE customer_id = $1 AND milestone_id = $2
        LIMIT 1
        "#,
    )
    .bind(customer_id)
    .bind(milestone_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.is_some())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn record_milestone_achievement(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    milestone_id: Uuid,
    ledger_entry_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO milestone_achievements (merchant_id, customer_id, milestone_id, ledger_entry_id)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (customer_id, milestone_id) DO NOTHING
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .bind(milestone_id)
    .bind(ledger_entry_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_customer_milestones(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<Vec<AchievedMilestone>, AppError> {
    let milestones = sqlx::query_as::<_, AchievedMilestone>(
        r#"
        SELECT mc.id, mc.merchant_id, mc.name, mc.milestone_type, mc.threshold,
               mc.reward_amount, ma.achieved_at
        FROM milestone_configs mc
        INNER JOIN milestone_achievements ma ON ma.milestone_id = mc.id
        WHERE mc.merchant_id = $1 AND ma.customer_id = $2
        ORDER BY ma.achieved_at ASC
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_all(pool)
    .await?;

    Ok(milestones)
}
