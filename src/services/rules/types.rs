use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Rule {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub rule_type: String,
    pub name: String,
    pub config: serde_json::Value,
    pub version: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RuleSnapshot {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub version: i32,
    pub config: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Campaign {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub name: String,
    pub campaign_type: String,
    pub config: serde_json::Value,
    pub base_rule_id: Option<Uuid>,
    pub multiplier: Option<f64>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CampaignSnapshot {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub config: serde_json::Value,
    pub multiplier: Option<f64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardRuleConfig {
    pub event_type: String,
    pub conditions: Vec<Condition>,
    pub action: RewardAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardAction {
    pub bucket_type: String,
    pub calculation: String,
    pub value: f64,
    pub max_amount: Option<f64>,
    pub expiry_days: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationContext {
    pub merchant_id: Uuid,
    pub event_type: String,
    pub event_payload: serde_json::Value,
    pub order_amount: Option<f64>,
    pub payment_method: Option<String>,
    pub is_cod: bool,
    pub collections: Vec<String>,
    pub customer_tags: Vec<String>,
    pub is_first_order: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvaluationResult {
    pub matched: bool,
    pub rule_snapshot_id: Option<Uuid>,
    pub campaign_snapshot_id: Option<Uuid>,
    pub earning_unit: f64,
    pub currency_equivalent: f64,
    pub conversion_rate: f64,
    pub bucket_type: String,
    pub expiry_days: Option<i32>,
    pub constraints: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct CreateRuleRequest {
    pub merchant_id: Uuid,
    pub rule_type: String,
    pub name: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRuleRequest {
    pub name: Option<String>,
    pub config: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateRequest {
    pub context: EvaluationContext,
}

#[derive(Debug, Deserialize)]
pub struct ListRulesQuery {
    pub merchant_id: Uuid,
    pub rule_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub merchant_id: Uuid,
    pub name: String,
    pub campaign_type: String,
    pub config: serde_json::Value,
    pub base_rule_id: Option<Uuid>,
    pub multiplier: Option<f64>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ListCampaignsQuery {
    pub merchant_id: Uuid,
    pub active_only: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RulePerformance {
    pub rule_id: Uuid,
    pub total_entries: i64,
    pub total_value: f64,
    pub unique_customers: i64,
}
