use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::types::{CampaignCalendarEntry, CampaignPerformance};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_campaign_calendar(
    pool: &PgPool,
    merchant_id: Uuid,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<Vec<CampaignCalendarEntry>, AppError> {
    let now = Utc::now();

    let rows = sqlx::query_as::<_, (Uuid, String, String, Option<f64>, DateTime<Utc>, DateTime<Utc>, bool)>(
        r#"
        SELECT id, name, campaign_type, multiplier, starts_at, ends_at, is_active
        FROM campaigns
        WHERE merchant_id = $1
          AND starts_at < $3
          AND ends_at > $2
        ORDER BY starts_at ASC
        "#,
    )
    .bind(merchant_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;

    let entries = rows
        .into_iter()
        .map(|(id, name, campaign_type, multiplier, starts_at, ends_at, is_active)| {
            let is_currently_running = is_active && starts_at <= now && ends_at > now;
            CampaignCalendarEntry {
                id,
                name,
                campaign_type,
                multiplier,
                starts_at,
                ends_at,
                is_active,
                is_currently_running,
            }
        })
        .collect();

    Ok(entries)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_campaign_performance(
    pool: &PgPool,
    campaign_id: Uuid,
) -> Result<CampaignPerformance, AppError> {
    let campaign = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT name
        FROM campaigns
        WHERE id = $1
        "#,
    )
    .bind(campaign_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("campaign {campaign_id} not found")))?;

    let stats = sqlx::query_as::<_, (i64, f64, i64)>(
        r#"
        SELECT
            COALESCE(COUNT(le.id), 0) AS total_entries,
            COALESCE(SUM(le.currency_equivalent), 0.0) AS total_value,
            COALESCE(COUNT(DISTINCT w.customer_id), 0) AS unique_customers
        FROM campaign_snapshots cs
        JOIN ledger_entries le ON le.campaign_snapshot_id = cs.id
        JOIN wallets w ON w.id = le.wallet_id
        WHERE cs.campaign_id = $1
        "#,
    )
    .bind(campaign_id)
    .fetch_one(pool)
    .await?;

    let (total_entries, total_value, unique_customers) = stats;
    let average_reward = if total_entries > 0 {
        total_value / total_entries as f64
    } else {
        0.0
    };

    Ok(CampaignPerformance {
        campaign_id,
        name: campaign.0,
        total_entries,
        total_value,
        unique_customers,
        average_reward,
    })
}
