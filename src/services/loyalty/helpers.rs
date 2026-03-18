use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::storage;
use super::types::{
    CustomerTierResponse, LoyaltyTier, TierEvaluationResult, TierProgress,
};

#[derive(Debug, sqlx::FromRow)]
struct QualifyingValueRow {
    value: f64,
}

#[tracing::instrument(skip(pool), err(Debug))]
async fn get_qualifying_value_spend(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    evaluation_period_days: Option<i32>,
) -> Result<f64, AppError> {
    let row = match evaluation_period_days {
        Some(days) => {
            sqlx::query_as::<_, QualifyingValueRow>(
                r#"
                SELECT COALESCE(SUM(le.currency_equivalent), 0)::float8 AS value
                FROM ledger_entries le
                JOIN wallets w ON w.id = le.wallet_id
                WHERE w.merchant_id = $1
                  AND w.customer_id = $2
                  AND le.bucket_type = 'earned_credit'
                  AND le.movement_type = 'in'
                  AND le.created_at >= now() - make_interval(days => $3)
                "#,
            )
            .bind(merchant_id)
            .bind(customer_id)
            .bind(days)
            .fetch_one(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, QualifyingValueRow>(
                r#"
                SELECT COALESCE(SUM(le.currency_equivalent), 0)::float8 AS value
                FROM ledger_entries le
                JOIN wallets w ON w.id = le.wallet_id
                WHERE w.merchant_id = $1
                  AND w.customer_id = $2
                  AND le.bucket_type = 'earned_credit'
                  AND le.movement_type = 'in'
                "#,
            )
            .bind(merchant_id)
            .bind(customer_id)
            .fetch_one(pool)
            .await?
        }
    };

    Ok(row.value)
}

#[tracing::instrument(skip(pool), err(Debug))]
async fn get_qualifying_value_points(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    evaluation_period_days: Option<i32>,
) -> Result<f64, AppError> {
    let row = match evaluation_period_days {
        Some(days) => {
            sqlx::query_as::<_, QualifyingValueRow>(
                r#"
                SELECT COALESCE(SUM(le.earning_unit), 0)::float8 AS value
                FROM ledger_entries le
                JOIN wallets w ON w.id = le.wallet_id
                WHERE w.merchant_id = $1
                  AND w.customer_id = $2
                  AND le.movement_type = 'in'
                  AND le.created_at >= now() - make_interval(days => $3)
                "#,
            )
            .bind(merchant_id)
            .bind(customer_id)
            .bind(days)
            .fetch_one(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, QualifyingValueRow>(
                r#"
                SELECT COALESCE(SUM(le.earning_unit), 0)::float8 AS value
                FROM ledger_entries le
                JOIN wallets w ON w.id = le.wallet_id
                WHERE w.merchant_id = $1
                  AND w.customer_id = $2
                  AND le.movement_type = 'in'
                "#,
            )
            .bind(merchant_id)
            .bind(customer_id)
            .fetch_one(pool)
            .await?
        }
    };

    Ok(row.value)
}

#[tracing::instrument(skip(pool), err(Debug))]
async fn get_qualifying_value_order_count(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<f64, AppError> {
    let row = sqlx::query_as::<_, QualifyingValueRow>(
        r#"
        SELECT COALESCE(total_orders, 0)::float8 AS value
        FROM customer_order_stats
        WHERE merchant_id = $1 AND customer_id = $2
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.value).unwrap_or(0.0))
}

fn find_qualifying_tier(tiers: &[LoyaltyTier], qualifying_value: f64) -> Option<LoyaltyTier> {
    tiers
        .iter()
        .rev()
        .find(|t| qualifying_value >= t.threshold)
        .cloned()
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn evaluate_tier(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<TierEvaluationResult, AppError> {
    let program = storage::get_program(pool, merchant_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "no loyalty program found for merchant {merchant_id}"
            ))
        })?;

    let tiers = storage::get_tiers(pool, program.id).await?;
    if tiers.is_empty() {
        return Ok(TierEvaluationResult {
            customer_id,
            current_tier: None,
            new_tier: None,
            changed: false,
            direction: None,
        });
    }

    let qualifying_value = match program.evaluation_criteria.as_str() {
        "spend" => {
            get_qualifying_value_spend(
                pool,
                merchant_id,
                customer_id,
                program.evaluation_period_days,
            )
            .await?
        }
        "points" => {
            get_qualifying_value_points(
                pool,
                merchant_id,
                customer_id,
                program.evaluation_period_days,
            )
            .await?
        }
        "order_count" => {
            get_qualifying_value_order_count(pool, merchant_id, customer_id).await?
        }
        other => {
            return Err(AppError::BadRequest(format!(
                "unsupported evaluation criteria: {other}"
            )));
        }
    };

    let new_tier = find_qualifying_tier(&tiers, qualifying_value);

    let current_customer_tier = storage::get_customer_tier(pool, customer_id, merchant_id).await?;
    let current_tier = match &current_customer_tier {
        Some(ct) => storage::get_tier_by_id(pool, ct.tier_id).await?,
        None => None,
    };

    let changed = match (&current_tier, &new_tier) {
        (None, None) => false,
        (None, Some(_)) => true,
        (Some(_), None) => true,
        (Some(curr), Some(next)) => curr.id != next.id,
    };

    let direction = if changed {
        match (&current_tier, &new_tier) {
            (None, Some(_)) => Some("upgrade".to_string()),
            (Some(_), None) => Some("downgrade".to_string()),
            (Some(curr), Some(next)) => {
                if next.rank > curr.rank {
                    Some("upgrade".to_string())
                } else {
                    Some("downgrade".to_string())
                }
            }
            _ => None,
        }
    } else {
        None
    };

    if changed {
        if let Some(ref tier) = new_tier {
            storage::upsert_customer_tier(
                pool,
                customer_id,
                merchant_id,
                tier.id,
                qualifying_value,
            )
            .await?;
        }
    }

    Ok(TierEvaluationResult {
        customer_id,
        current_tier,
        new_tier,
        changed,
        direction,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_customer_tier_info(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<Option<CustomerTierResponse>, AppError> {
    let customer_tier = match storage::get_customer_tier(pool, customer_id, merchant_id).await? {
        Some(ct) => ct,
        None => return Ok(None),
    };

    let tier = storage::get_tier_by_id(pool, customer_tier.tier_id)
        .await?
        .ok_or_else(|| {
            AppError::Internal(format!("tier {} not found for customer tier", customer_tier.tier_id))
        })?;

    let program = storage::get_program_by_id(pool, tier.program_id)
        .await?
        .ok_or_else(|| {
            AppError::Internal(format!("program {} not found for tier", tier.program_id))
        })?;

    let tiers = storage::get_tiers(pool, program.id).await?;
    let progress_to_next = tiers
        .iter()
        .find(|t| t.rank > tier.rank)
        .map(|next_tier| {
            let percentage = if next_tier.threshold > 0.0 {
                (customer_tier.qualifying_value / next_tier.threshold * 100.0).min(100.0)
            } else {
                100.0
            };
            TierProgress {
                next_tier_name: next_tier.name.clone(),
                current_value: customer_tier.qualifying_value,
                threshold: next_tier.threshold,
                percentage,
            }
        });

    Ok(Some(CustomerTierResponse {
        customer: customer_tier,
        tier,
        program,
        progress_to_next,
    }))
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_earn_multiplier(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<f64, AppError> {
    let program = match storage::get_program(pool, merchant_id).await? {
        Some(p) => p,
        None => return Ok(1.0),
    };

    let customer_tier = match storage::get_customer_tier(pool, customer_id, merchant_id).await? {
        Some(ct) => ct,
        None => return Ok(1.0),
    };

    let tier = match storage::get_tier_by_id(pool, customer_tier.tier_id).await? {
        Some(t) if t.program_id == program.id => t,
        _ => return Ok(1.0),
    };

    Ok(tier.earn_rate_multiplier)
}
