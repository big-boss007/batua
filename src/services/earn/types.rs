use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ProcessEarnRequest {
    pub event_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct EarnResult {
    pub event_id: Uuid,
    pub customer_id: Uuid,
    pub wallet_id: Uuid,
    pub entries_created: Vec<EarnEntry>,
    pub is_cod: bool,
}

#[derive(Debug, Serialize)]
pub struct EarnEntry {
    pub ledger_entry_id: Uuid,
    pub bucket_type: String,
    pub earning_unit: f64,
    pub currency_equivalent: f64,
    pub movement_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ManualCreditRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    pub amount: f64,
    pub bucket_type: String,
    pub reason: String,
    pub actor_id: String,
}

#[derive(Debug, Serialize)]
pub struct ManualCreditResult {
    pub ledger_entry_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct ProcessBirthdayBonusRequest {
    pub merchant_id: Uuid,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct BirthdayBonusResult {
    pub merchant_id: Uuid,
    pub processed: i32,
    pub credited: i32,
    pub skipped: i32,
    pub entries: Vec<BirthdayBonusEntry>,
}

#[derive(Debug, Serialize)]
pub struct BirthdayBonusEntry {
    pub customer_id: Uuid,
    pub customer_name: Option<String>,
    pub amount: f64,
    pub ledger_entry_id: Uuid,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct MilestoneConfig {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub name: String,
    pub milestone_type: String,
    pub threshold: f64,
    pub reward_amount: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMilestoneRequest {
    pub merchant_id: Uuid,
    pub name: String,
    pub milestone_type: String,
    pub threshold: f64,
    pub reward_amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct CheckMilestonesRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct MilestoneCheckResult {
    pub customer_id: Uuid,
    pub milestones_achieved: Vec<MilestoneAchievementEntry>,
}

#[derive(Debug, Serialize)]
pub struct MilestoneAchievementEntry {
    pub milestone_name: String,
    pub reward_amount: f64,
    pub ledger_entry_id: Uuid,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct AchievedMilestone {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub name: String,
    pub milestone_type: String,
    pub threshold: f64,
    pub reward_amount: f64,
    pub achieved_at: DateTime<Utc>,
}
