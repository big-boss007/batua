use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::types::{
    AchievedMilestone, CreateMilestoneRequest, CreateStreakConfigRequest, CustomerMembership,
    MilestoneConfig, NewsletterSignupCount, SpinWheelConfig, SpinWheelSegment, StreakConfig,
};

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

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn has_newsletter_signup(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<bool, AppError> {
    let row: Option<(i32,)> = sqlx::query_as(
        r#"
        SELECT 1 AS one
        FROM newsletter_signups
        WHERE merchant_id = $1 AND customer_id = $2
        LIMIT 1
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.is_some())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn record_newsletter_signup(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    ledger_entry_id: Uuid,
    email: &str,
    source: &str,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO newsletter_signups (merchant_id, customer_id, ledger_entry_id, email, source)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (merchant_id, customer_id) DO NOTHING
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .bind(ledger_entry_id)
    .bind(email)
    .bind(source)
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_newsletter_signup_count(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<NewsletterSignupCount, AppError> {
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)::int8 AS count
        FROM newsletter_signups
        WHERE merchant_id = $1
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    Ok(NewsletterSignupCount {
        merchant_id,
        count: row.0,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_streak_config(
    pool: &PgPool,
    req: &CreateStreakConfigRequest,
) -> Result<StreakConfig, AppError> {
    let config = sqlx::query_as::<_, StreakConfig>(
        r#"
        INSERT INTO streak_configs (merchant_id, name, required_orders, window_days, reward_amount)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(req.merchant_id)
    .bind(&req.name)
    .bind(req.required_orders)
    .bind(req.window_days)
    .bind(req.reward_amount)
    .fetch_one(pool)
    .await?;

    Ok(config)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_active_streak_configs(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<StreakConfig>, AppError> {
    let configs = sqlx::query_as::<_, StreakConfig>(
        r#"
        SELECT id, merchant_id, name, required_orders, window_days, reward_amount, is_active, created_at
        FROM streak_configs
        WHERE merchant_id = $1 AND is_active = true
        ORDER BY required_orders ASC
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(configs)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn count_recent_orders(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    days: i32,
) -> Result<i64, AppError> {
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT le.payment_reference)::int8
        FROM ledger_entries le
        JOIN wallets w ON le.wallet_id = w.id
        WHERE w.merchant_id = $1
            AND w.customer_id = $2
            AND le.movement_type = 'in'
            AND le.bucket_type = 'earned_credit'
            AND le.payment_reference LIKE 'order:%'
            AND le.created_at > NOW() - ($3 || ' days')::interval
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .bind(days.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn has_streak_achievement_in_window(
    pool: &PgPool,
    customer_id: Uuid,
    streak_config_id: Uuid,
    window_start: DateTime<Utc>,
) -> Result<bool, AppError> {
    let row: Option<(i32,)> = sqlx::query_as(
        r#"
        SELECT 1 AS one
        FROM streak_achievements
        WHERE customer_id = $1
            AND streak_config_id = $2
            AND window_start::date = $3::date
        LIMIT 1
        "#,
    )
    .bind(customer_id)
    .bind(streak_config_id)
    .bind(window_start)
    .fetch_optional(pool)
    .await?;

    Ok(row.is_some())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn record_streak_achievement(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    streak_config_id: Uuid,
    ledger_entry_id: Uuid,
    window_start: DateTime<Utc>,
    window_end: DateTime<Utc>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO streak_achievements (merchant_id, customer_id, streak_config_id, ledger_entry_id, window_start, window_end)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .bind(streak_config_id)
    .bind(ledger_entry_id)
    .bind(window_start)
    .bind(window_end)
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_wheel_config(
    pool: &PgPool,
    merchant_id: Uuid,
    name: &str,
    daily_spin_limit: i32,
) -> Result<SpinWheelConfig, AppError> {
    let config = sqlx::query_as::<_, SpinWheelConfig>(
        r#"
        INSERT INTO spin_wheel_configs (merchant_id, name, daily_spin_limit)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(merchant_id)
    .bind(name)
    .bind(daily_spin_limit)
    .fetch_one(pool)
    .await?;

    Ok(config)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_wheel_segment(
    pool: &PgPool,
    wheel_id: Uuid,
    label: &str,
    reward_amount: f64,
    probability: f64,
    color: &str,
    position: i32,
) -> Result<SpinWheelSegment, AppError> {
    let segment = sqlx::query_as::<_, SpinWheelSegment>(
        r#"
        INSERT INTO spin_wheel_segments (wheel_id, label, reward_amount, probability, color, position)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(wheel_id)
    .bind(label)
    .bind(reward_amount)
    .bind(probability)
    .bind(color)
    .bind(position)
    .fetch_one(pool)
    .await?;

    Ok(segment)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_wheel_config(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Option<SpinWheelConfig>, AppError> {
    let config = sqlx::query_as::<_, SpinWheelConfig>(
        r#"
        SELECT id, merchant_id, name, is_active, daily_spin_limit, created_at
        FROM spin_wheel_configs
        WHERE merchant_id = $1
        "#,
    )
    .bind(merchant_id)
    .fetch_optional(pool)
    .await?;

    Ok(config)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_wheel_segments(
    pool: &PgPool,
    wheel_id: Uuid,
) -> Result<Vec<SpinWheelSegment>, AppError> {
    let segments = sqlx::query_as::<_, SpinWheelSegment>(
        r#"
        SELECT id, wheel_id, label, reward_amount, probability, color, position, created_at
        FROM spin_wheel_segments
        WHERE wheel_id = $1
        ORDER BY position ASC
        "#,
    )
    .bind(wheel_id)
    .fetch_all(pool)
    .await?;

    Ok(segments)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn count_spins_today(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<i64, AppError> {
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)::int8
        FROM spin_results
        WHERE merchant_id = $1
            AND customer_id = $2
            AND spun_at >= CURRENT_DATE
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn record_spin_result(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    segment_id: Uuid,
    reward_amount: f64,
    ledger_entry_id: Option<Uuid>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO spin_results (merchant_id, customer_id, segment_id, reward_amount, ledger_entry_id)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .bind(segment_id)
    .bind(reward_amount)
    .bind(ledger_entry_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn subscribe_customer(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    tier_id: Uuid,
    expires_at: DateTime<Utc>,
) -> Result<CustomerMembership, AppError> {
    let membership = sqlx::query_as::<_, CustomerMembership>(
        r#"
        INSERT INTO customer_memberships (merchant_id, customer_id, tier_id, status, expires_at)
        VALUES ($1, $2, $3, 'active', $4)
        RETURNING *
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .bind(tier_id)
    .bind(expires_at)
    .fetch_one(pool)
    .await?;

    Ok(membership)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_customer_membership(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<Option<CustomerMembership>, AppError> {
    let membership = sqlx::query_as::<_, CustomerMembership>(
        r#"
        SELECT id, merchant_id, customer_id, tier_id, status, started_at, expires_at,
               renewed_count, cancelled_at, created_at
        FROM customer_memberships
        WHERE merchant_id = $1 AND customer_id = $2
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await?;

    Ok(membership)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_customer_membership_by_id(
    pool: &PgPool,
    membership_id: Uuid,
) -> Result<CustomerMembership, AppError> {
    let membership = sqlx::query_as::<_, CustomerMembership>(
        r#"
        SELECT id, merchant_id, customer_id, tier_id, status, started_at, expires_at,
               renewed_count, cancelled_at, created_at
        FROM customer_memberships
        WHERE id = $1
        "#,
    )
    .bind(membership_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            AppError::NotFound(format!("membership {membership_id} not found"))
        }
        other => AppError::Database(other),
    })?;

    Ok(membership)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn renew_membership(
    pool: &PgPool,
    membership_id: Uuid,
    new_expires_at: DateTime<Utc>,
) -> Result<CustomerMembership, AppError> {
    let membership = sqlx::query_as::<_, CustomerMembership>(
        r#"
        UPDATE customer_memberships
        SET expires_at = $2,
            renewed_count = renewed_count + 1,
            status = 'active'
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(membership_id)
    .bind(new_expires_at)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            AppError::NotFound(format!("membership {membership_id} not found"))
        }
        other => AppError::Database(other),
    })?;

    Ok(membership)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn cancel_membership(
    pool: &PgPool,
    membership_id: Uuid,
) -> Result<CustomerMembership, AppError> {
    let membership = sqlx::query_as::<_, CustomerMembership>(
        r#"
        UPDATE customer_memberships
        SET status = 'cancelled',
            cancelled_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(membership_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            AppError::NotFound(format!("membership {membership_id} not found"))
        }
        other => AppError::Database(other),
    })?;

    Ok(membership)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn expire_membership(
    pool: &PgPool,
    membership_id: Uuid,
) -> Result<CustomerMembership, AppError> {
    let membership = sqlx::query_as::<_, CustomerMembership>(
        r#"
        UPDATE customer_memberships
        SET status = 'expired'
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(membership_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            AppError::NotFound(format!("membership {membership_id} not found"))
        }
        other => AppError::Database(other),
    })?;

    Ok(membership)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_expired_memberships(
    pool: &PgPool,
) -> Result<Vec<CustomerMembership>, AppError> {
    let memberships = sqlx::query_as::<_, CustomerMembership>(
        r#"
        SELECT id, merchant_id, customer_id, tier_id, status, started_at, expires_at,
               renewed_count, cancelled_at, created_at
        FROM customer_memberships
        WHERE status = 'active' AND expires_at < now()
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(memberships)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_memberships_by_merchant(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<CustomerMembership>, AppError> {
    let memberships = sqlx::query_as::<_, CustomerMembership>(
        r#"
        SELECT id, merchant_id, customer_id, tier_id, status, started_at, expires_at,
               renewed_count, cancelled_at, created_at
        FROM customer_memberships
        WHERE merchant_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(memberships)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_enriched_memberships(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<super::types::EnrichedMembership>, AppError> {
    let rows = sqlx::query_as::<_, super::types::EnrichedMembership>(
        r#"
        SELECT cm.id, cm.merchant_id, cm.customer_id, cm.tier_id, cm.status,
               cm.started_at, cm.expires_at, cm.renewed_count, cm.cancelled_at, cm.created_at,
               c.name AS customer_name, c.phone AS customer_phone,
               lt.name AS tier_name,
               lt.earn_rate_multiplier::float8 AS earn_rate_multiplier
        FROM customer_memberships cm
        JOIN customers c ON c.id = cm.customer_id
        JOIN loyalty_tiers lt ON lt.id = cm.tier_id
        WHERE cm.merchant_id = $1
        ORDER BY cm.created_at DESC
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
