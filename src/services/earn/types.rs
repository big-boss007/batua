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

#[derive(Debug, Deserialize)]
pub struct NewsletterSignupRequest {
    pub merchant_id: Uuid,
    pub email: String,
    pub phone: Option<String>,
    pub customer_id: Option<Uuid>,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct NewsletterSignupResult {
    pub customer_id: Uuid,
    pub email: String,
    pub rewarded: bool,
    pub already_subscribed: bool,
    pub ledger_entry_id: Option<Uuid>,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct NewsletterSignupCount {
    pub merchant_id: Uuid,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
pub struct ProfileCompletionRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ProfileCompletionResult {
    pub customer_id: Uuid,
    pub fields_complete: Vec<String>,
    pub fields_missing: Vec<String>,
    pub completion_pct: f64,
    pub already_rewarded: bool,
    pub rewarded: bool,
    pub amount: f64,
    pub ledger_entry_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct StreakConfig {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub name: String,
    pub required_orders: i32,
    pub window_days: i32,
    pub reward_amount: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStreakConfigRequest {
    pub merchant_id: Uuid,
    pub name: String,
    pub required_orders: i32,
    pub window_days: i32,
    pub reward_amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct CheckStreakRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct StreakCheckResult {
    pub customer_id: Uuid,
    pub streaks_achieved: Vec<StreakAchievementEntry>,
    pub active_streaks: Vec<ActiveStreak>,
}

#[derive(Debug, Serialize)]
pub struct StreakAchievementEntry {
    pub streak_name: String,
    pub reward_amount: f64,
    pub ledger_entry_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ActiveStreak {
    pub streak_name: String,
    pub required_orders: i32,
    pub orders_in_window: i64,
    pub window_days: i32,
    pub progress_pct: f64,
}
