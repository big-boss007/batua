use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::types::{
    CreateProgramRequest, CreateTierRequest, CustomerTier, LoyaltyProgram, LoyaltyTier,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_program(
    pool: &PgPool,
    req: &CreateProgramRequest,
) -> Result<LoyaltyProgram, AppError> {
    let program = sqlx::query_as::<_, LoyaltyProgram>(
        r#"
        INSERT INTO loyalty_programs (merchant_id, name, evaluation_criteria, evaluation_period_days)
        VALUES ($1, $2, $3, $4)
        RETURNING id, merchant_id, name, evaluation_criteria, evaluation_period_days,
                  is_active, created_at, updated_at
        "#,
    )
    .bind(req.merchant_id)
    .bind(&req.name)
    .bind(&req.evaluation_criteria)
    .bind(req.evaluation_period_days)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("loyalty program already exists for this merchant".to_string())
        }
        other => AppError::Database(other),
    })?;

    Ok(program)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_program(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Option<LoyaltyProgram>, AppError> {
    let program = sqlx::query_as::<_, LoyaltyProgram>(
        r#"
        SELECT id, merchant_id, name, evaluation_criteria, evaluation_period_days,
               is_active, created_at, updated_at
        FROM loyalty_programs
        WHERE merchant_id = $1
        "#,
    )
    .bind(merchant_id)
    .fetch_optional(pool)
    .await?;

    Ok(program)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_program_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<LoyaltyProgram>, AppError> {
    let program = sqlx::query_as::<_, LoyaltyProgram>(
        r#"
        SELECT id, merchant_id, name, evaluation_criteria, evaluation_period_days,
               is_active, created_at, updated_at
        FROM loyalty_programs
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(program)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_program(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    evaluation_criteria: &str,
    evaluation_period_days: Option<i32>,
    is_active: bool,
) -> Result<LoyaltyProgram, AppError> {
    let program = sqlx::query_as::<_, LoyaltyProgram>(
        r#"
        UPDATE loyalty_programs
        SET name = $2, evaluation_criteria = $3, evaluation_period_days = $4,
            is_active = $5, updated_at = now()
        WHERE id = $1
        RETURNING id, merchant_id, name, evaluation_criteria, evaluation_period_days,
                  is_active, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(evaluation_criteria)
    .bind(evaluation_period_days)
    .bind(is_active)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("loyalty program {id} not found")))?;

    Ok(program)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_tier(
    pool: &PgPool,
    req: &CreateTierRequest,
) -> Result<LoyaltyTier, AppError> {
    let tier = sqlx::query_as::<_, LoyaltyTier>(
        r#"
        INSERT INTO loyalty_tiers (program_id, name, rank, threshold, earn_rate_multiplier, benefits)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, program_id, name, rank,
                  threshold::float8 AS threshold,
                  earn_rate_multiplier::float8 AS earn_rate_multiplier,
                  benefits, created_at
        "#,
    )
    .bind(req.program_id)
    .bind(&req.name)
    .bind(req.rank)
    .bind(req.threshold)
    .bind(req.earn_rate_multiplier)
    .bind(&req.benefits)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!(
                "tier with rank {} already exists for this program",
                req.rank
            ))
        }
        other => AppError::Database(other),
    })?;

    Ok(tier)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_tiers(
    pool: &PgPool,
    program_id: Uuid,
) -> Result<Vec<LoyaltyTier>, AppError> {
    let tiers = sqlx::query_as::<_, LoyaltyTier>(
        r#"
        SELECT id, program_id, name, rank,
               threshold::float8 AS threshold,
               earn_rate_multiplier::float8 AS earn_rate_multiplier,
               benefits, created_at
        FROM loyalty_tiers
        WHERE program_id = $1
        ORDER BY rank ASC
        "#,
    )
    .bind(program_id)
    .fetch_all(pool)
    .await?;

    Ok(tiers)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_tier_by_id(
    pool: &PgPool,
    tier_id: Uuid,
) -> Result<Option<LoyaltyTier>, AppError> {
    let tier = sqlx::query_as::<_, LoyaltyTier>(
        r#"
        SELECT id, program_id, name, rank,
               threshold::float8 AS threshold,
               earn_rate_multiplier::float8 AS earn_rate_multiplier,
               benefits, created_at
        FROM loyalty_tiers
        WHERE id = $1
        "#,
    )
    .bind(tier_id)
    .fetch_optional(pool)
    .await?;

    Ok(tier)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_customer_tier(
    pool: &PgPool,
    customer_id: Uuid,
    merchant_id: Uuid,
) -> Result<Option<CustomerTier>, AppError> {
    let ct = sqlx::query_as::<_, CustomerTier>(
        r#"
        SELECT id, customer_id, merchant_id, tier_id,
               qualifying_value::float8 AS qualifying_value,
               qualified_at, expires_at, created_at, updated_at
        FROM customer_tiers
        WHERE customer_id = $1 AND merchant_id = $2
        "#,
    )
    .bind(customer_id)
    .bind(merchant_id)
    .fetch_optional(pool)
    .await?;

    Ok(ct)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn upsert_customer_tier(
    pool: &PgPool,
    customer_id: Uuid,
    merchant_id: Uuid,
    tier_id: Uuid,
    qualifying_value: f64,
) -> Result<CustomerTier, AppError> {
    let ct = sqlx::query_as::<_, CustomerTier>(
        r#"
        INSERT INTO customer_tiers (customer_id, merchant_id, tier_id, qualifying_value, qualified_at)
        VALUES ($1, $2, $3, $4, now())
        ON CONFLICT (customer_id, merchant_id)
        DO UPDATE SET
            tier_id = $3,
            qualifying_value = $4,
            qualified_at = now(),
            updated_at = now()
        RETURNING id, customer_id, merchant_id, tier_id,
                  qualifying_value::float8 AS qualifying_value,
                  qualified_at, expires_at, created_at, updated_at
        "#,
    )
    .bind(customer_id)
    .bind(merchant_id)
    .bind(tier_id)
    .bind(qualifying_value)
    .fetch_one(pool)
    .await?;

    Ok(ct)
}

#[derive(Debug, sqlx::FromRow)]
pub struct TierDistributionRow {
    pub tier_name: String,
    pub customer_count: i64,
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_tier_distribution(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<(String, i64)>, AppError> {
    let rows = sqlx::query_as::<_, TierDistributionRow>(
        r#"
        SELECT lt.name AS tier_name, COUNT(ct.id)::int8 AS customer_count
        FROM loyalty_tiers lt
        JOIN loyalty_programs lp ON lp.id = lt.program_id
        LEFT JOIN customer_tiers ct ON ct.tier_id = lt.id AND ct.merchant_id = $1
        WHERE lp.merchant_id = $1
        GROUP BY lt.name, lt.rank
        ORDER BY lt.rank ASC
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.tier_name, r.customer_count)).collect())
}
