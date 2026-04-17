use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::ledger::types::BucketType;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum CodeCreationTrigger {
    OnRegistration,
    OnFirstPurchase,
}

impl Default for CodeCreationTrigger {
    fn default() -> Self {
        Self::OnRegistration
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ReferralProgram {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub referrer_reward_amount: f64,
    pub referee_reward_amount: f64,
    pub referrer_bucket_type: BucketType,
    pub referee_bucket_type: BucketType,
    pub max_referrals_per_customer: Option<i32>,
    pub is_active: bool,
    pub code_creation_trigger: CodeCreationTrigger,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ReferralCode {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    pub code: String,
    pub is_vanity: bool,
    pub is_creator: bool,
    pub commission_rate: Option<f64>,
    pub total_referrals: i32,
    pub total_conversions: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ReferralConversion {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub referral_code_id: Uuid,
    pub referrer_id: Uuid,
    pub referee_id: Uuid,
    pub order_id: Option<String>,
    pub referrer_entry_id: Option<Uuid>,
    pub referee_entry_id: Option<Uuid>,
    pub referee_ip: Option<String>,
    pub referee_device_fingerprint: Option<String>,
    pub is_suspicious: bool,
    pub fraud_signals: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProgramRequest {
    pub merchant_id: Uuid,
    pub referrer_reward_amount: f64,
    pub referee_reward_amount: f64,
    pub max_referrals_per_customer: Option<i32>,
    pub code_creation_trigger: Option<CodeCreationTrigger>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProgramRequest {
    pub referrer_reward_amount: Option<f64>,
    pub referee_reward_amount: Option<f64>,
    pub max_referrals_per_customer: Option<Option<i32>>,
    pub is_active: Option<bool>,
    pub code_creation_trigger: Option<CodeCreationTrigger>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ReferralCodeWithCustomer {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    pub code: String,
    pub is_vanity: bool,
    pub is_creator: bool,
    pub commission_rate: Option<f64>,
    pub total_referrals: i32,
    pub total_conversions: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub customer_phone: Option<String>,
    pub customer_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCodeRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    pub code: Option<String>,
    pub is_vanity: bool,
    pub is_creator: bool,
    pub commission_rate: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessReferralRequest {
    pub merchant_id: Uuid,
    pub referral_code: String,
    pub referee_id: Uuid,
    pub order_id: Option<String>,
    pub referee_ip: Option<String>,
    pub referee_device_fingerprint: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReferralResponse {
    pub conversion_id: Uuid,
    pub referrer_rewarded: bool,
    pub referee_rewarded: bool,
    pub fraud_signals: Vec<String>,
}

#[derive(Debug)]
pub struct FraudCheckResult {
    pub is_suspicious: bool,
    pub signals: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ReferralAnalytics {
    pub total_codes: i64,
    pub total_referrals: i64,
    pub total_conversions: i64,
    pub total_suspicious: i64,
    pub conversion_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}
