use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FestiveTemplate {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub default_multiplier: f64,
    pub default_duration_days: i32,
    pub suggested_start: Option<String>,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFromTemplateRequest {
    pub merchant_id: Uuid,
    pub template_name: String,
    pub base_rule_id: Uuid,
    pub multiplier: Option<f64>,
    pub starts_at: String,
    pub ends_at: String,
    pub custom_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CampaignCalendarEntry {
    pub id: Uuid,
    pub name: String,
    pub campaign_type: String,
    pub multiplier: Option<f64>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub is_active: bool,
    pub is_currently_running: bool,
}

#[derive(Debug, Serialize)]
pub struct CampaignPerformance {
    pub campaign_id: Uuid,
    pub name: String,
    pub total_entries: i64,
    pub total_value: f64,
    pub unique_customers: i64,
    pub average_reward: f64,
}

#[derive(Debug, Deserialize)]
pub struct CampaignCalendarQuery {
    pub merchant_id: Uuid,
    pub from: Option<String>,
    pub to: Option<String>,
}
