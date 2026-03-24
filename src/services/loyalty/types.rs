use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct LoyaltyProgram {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub name: String,
    pub evaluation_criteria: String,
    pub evaluation_period_days: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct LoyaltyTier {
    pub id: Uuid,
    pub program_id: Uuid,
    pub name: String,
    pub rank: i32,
    pub threshold: f64,
    pub earn_rate_multiplier: f64,
    pub benefits: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CustomerTier {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub merchant_id: Uuid,
    pub tier_id: Uuid,
    pub qualifying_value: f64,
    pub qualified_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProgramRequest {
    pub merchant_id: Uuid,
    pub name: String,
    pub evaluation_criteria: String,
    pub evaluation_period_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTierRequest {
    pub program_id: Uuid,
    pub name: String,
    pub rank: i32,
    pub threshold: f64,
    pub earn_rate_multiplier: f64,
    pub benefits: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProgramRequest {
    pub name: String,
    pub evaluation_criteria: String,
    pub evaluation_period_days: Option<i32>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTierRequest {
    pub name: Option<String>,
    pub rank: Option<i32>,
    pub threshold: Option<f64>,
    pub earn_rate_multiplier: Option<f64>,
    pub benefits: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct TierEvaluationResult {
    pub customer_id: Uuid,
    pub current_tier: Option<LoyaltyTier>,
    pub new_tier: Option<LoyaltyTier>,
    pub changed: bool,
    pub direction: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CustomerTierResponse {
    pub customer: CustomerTier,
    pub tier: LoyaltyTier,
    pub program: LoyaltyProgram,
    pub progress_to_next: Option<TierProgress>,
}

#[derive(Debug, Serialize)]
pub struct TierProgress {
    pub next_tier_name: String,
    pub current_value: f64,
    pub threshold: f64,
    pub percentage: f64,
}
