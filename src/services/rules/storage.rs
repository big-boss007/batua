use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::types::{
    Campaign, CampaignSnapshot, CreateCampaignRequest, CreateRuleRequest, Rule, RulePerformance,
    RuleSnapshot,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_rule(pool: &PgPool, req: &CreateRuleRequest) -> Result<Rule, AppError> {
    let rule = sqlx::query_as::<_, Rule>(
        r#"
        INSERT INTO rules (merchant_id, rule_type, name, config)
        VALUES ($1, $2, $3, $4)
        RETURNING id, merchant_id, rule_type, name, config, version, is_active, created_at, updated_at
        "#,
    )
    .bind(&req.merchant_id)
    .bind(&req.rule_type)
    .bind(&req.name)
    .bind(&req.config)
    .fetch_one(pool)
    .await?;

    Ok(rule)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_rule(
    pool: &PgPool,
    id: Uuid,
    name: Option<&str>,
    config: &serde_json::Value,
    is_active: Option<bool>,
) -> Result<Rule, AppError> {
    let rule = sqlx::query_as::<_, Rule>(
        r#"
        UPDATE rules
        SET name = COALESCE($2, name),
            config = $3,
            is_active = COALESCE($4, is_active),
            version = version + 1,
            updated_at = now()
        WHERE id = $1
        RETURNING id, merchant_id, rule_type, name, config, version, is_active, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(config)
    .bind(is_active)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("rule {id} not found")))?;

    Ok(rule)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_rule(pool: &PgPool, id: Uuid) -> Result<Rule, AppError> {
    let rule = sqlx::query_as::<_, Rule>(
        r#"
        SELECT id, merchant_id, rule_type, name, config, version, is_active, created_at, updated_at
        FROM rules
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("rule {id} not found")))?;

    Ok(rule)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_active_rules(
    pool: &PgPool,
    merchant_id: Uuid,
    rule_type: &str,
) -> Result<Vec<Rule>, AppError> {
    let rules = sqlx::query_as::<_, Rule>(
        r#"
        SELECT id, merchant_id, rule_type, name, config, version, is_active, created_at, updated_at
        FROM rules
        WHERE merchant_id = $1
          AND rule_type = $2
        ORDER BY is_active DESC, created_at ASC
        "#,
    )
    .bind(merchant_id)
    .bind(rule_type)
    .fetch_all(pool)
    .await?;

    Ok(rules)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_rule_snapshot(pool: &PgPool, rule: &Rule) -> Result<RuleSnapshot, AppError> {
    let snapshot = sqlx::query_as::<_, RuleSnapshot>(
        r#"
        INSERT INTO rule_snapshots (rule_id, version, config)
        VALUES ($1, $2, $3)
        RETURNING id, rule_id, version, config, created_at
        "#,
    )
    .bind(rule.id)
    .bind(rule.version)
    .bind(&rule.config)
    .fetch_one(pool)
    .await?;

    Ok(snapshot)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_active_campaigns(
    pool: &PgPool,
    merchant_id: Uuid,
    at: DateTime<Utc>,
) -> Result<Vec<Campaign>, AppError> {
    let campaigns = sqlx::query_as::<_, Campaign>(
        r#"
        SELECT id, merchant_id, name, campaign_type, config, base_rule_id, multiplier,
               starts_at, ends_at, is_active, created_at, updated_at
        FROM campaigns
        WHERE merchant_id = $1
          AND is_active = true
          AND starts_at <= $2
          AND ends_at > $2
        ORDER BY created_at ASC
        "#,
    )
    .bind(merchant_id)
    .bind(at)
    .fetch_all(pool)
    .await?;

    Ok(campaigns)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_campaign_snapshot(
    pool: &PgPool,
    campaign: &Campaign,
) -> Result<CampaignSnapshot, AppError> {
    let snapshot = sqlx::query_as::<_, CampaignSnapshot>(
        r#"
        INSERT INTO campaign_snapshots (campaign_id, config, multiplier)
        VALUES ($1, $2, $3)
        RETURNING id, campaign_id, config, multiplier, created_at
        "#,
    )
    .bind(campaign.id)
    .bind(&campaign.config)
    .bind(campaign.multiplier)
    .fetch_one(pool)
    .await?;

    Ok(snapshot)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_campaign(
    pool: &PgPool,
    req: &CreateCampaignRequest,
) -> Result<Campaign, AppError> {
    let campaign = sqlx::query_as::<_, Campaign>(
        r#"
        INSERT INTO campaigns (merchant_id, name, campaign_type, config, base_rule_id, multiplier, starts_at, ends_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, merchant_id, name, campaign_type, config, base_rule_id, multiplier,
                  starts_at, ends_at, is_active, created_at, updated_at
        "#,
    )
    .bind(&req.merchant_id)
    .bind(&req.name)
    .bind(&req.campaign_type)
    .bind(&req.config)
    .bind(req.base_rule_id)
    .bind(req.multiplier)
    .bind(req.starts_at)
    .bind(req.ends_at)
    .fetch_one(pool)
    .await?;

    Ok(campaign)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_campaigns(
    pool: &PgPool,
    merchant_id: Uuid,
    active_only: bool,
) -> Result<Vec<Campaign>, AppError> {
    let campaigns = if active_only {
        let now = Utc::now();
        sqlx::query_as::<_, Campaign>(
            r#"
            SELECT id, merchant_id, name, campaign_type, config, base_rule_id, multiplier,
                   starts_at, ends_at, is_active, created_at, updated_at
            FROM campaigns
            WHERE merchant_id = $1
              AND is_active = true
              AND starts_at <= $2
              AND ends_at > $2
            ORDER BY created_at DESC
            "#,
        )
        .bind(merchant_id)
        .bind(now)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, Campaign>(
            r#"
            SELECT id, merchant_id, name, campaign_type, config, base_rule_id, multiplier,
                   starts_at, ends_at, is_active, created_at, updated_at
            FROM campaigns
            WHERE merchant_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(merchant_id)
        .fetch_all(pool)
        .await?
    };

    Ok(campaigns)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_rule_performance(
    pool: &PgPool,
    rule_id: Uuid,
) -> Result<RulePerformance, AppError> {
    let row: (i64, f64, i64) = sqlx::query_as(
        r#"
        SELECT
            COUNT(le.id)::int8,
            COALESCE(SUM(le.currency_equivalent), 0)::float8,
            COUNT(DISTINCT le.wallet_id)::int8
        FROM rule_snapshots rs
        JOIN ledger_entries le ON le.rule_snapshot_id = rs.id
        WHERE rs.rule_id = $1
        "#,
    )
    .bind(rule_id)
    .fetch_one(pool)
    .await?;

    Ok(RulePerformance {
        rule_id,
        total_entries: row.0,
        total_value: row.1,
        unique_customers: row.2,
    })
}
