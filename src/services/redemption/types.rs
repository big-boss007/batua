use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::ledger::types::BucketType;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "redemption_state", rename_all = "lowercase")]
pub enum RedemptionState {
    Initiated,
    Validating,
    Rejected,
    Committed,
    Applied,
    Failed,
    Compensated,
    Completed,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RedemptionRequest {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub wallet_id: Uuid,
    pub requested_amount: f64,
    pub eligible_amount: Option<f64>,
    pub applied_amount: Option<f64>,
    pub order_id: String,
    pub order_amount: f64,
    pub payment_method: Option<String>,
    pub state: RedemptionState,
    pub debit_entry_id: Option<Uuid>,
    pub compensation_entry_id: Option<Uuid>,
    pub shopify_discount_id: Option<String>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct WalletPolicy {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub bucket_type: BucketType,
    pub min_redemption: Option<f64>,
    pub step_size: Option<f64>,
    pub max_per_order_pct: Option<f64>,
    pub max_per_order_fixed: Option<f64>,
    pub stackable_with_discounts: bool,
    pub default_expiry_days: Option<i32>,
    pub excluded_payment_methods: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct InitiateRedemptionRequest {
    pub wallet_id: Uuid,
    pub order_id: String,
    pub order_amount: f64,
    pub payment_method: Option<String>,
    pub requested_amount: f64,
    pub discount_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BucketEligibility {
    pub bucket_type: BucketType,
    pub eligible_amount: f64,
    pub constraints: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct RedemptionEligibility {
    pub total_eligible: f64,
    pub buckets: Vec<BucketEligibility>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BucketDebit {
    pub bucket_type: BucketType,
    pub amount: f64,
    pub entry_id: Uuid,
}

#[derive(Debug, Clone, Serialize)]
pub struct RedemptionResponse {
    pub redemption_id: Uuid,
    pub state: RedemptionState,
    pub applied_amount: Option<f64>,
    pub buckets_debited: Vec<BucketDebit>,
}

#[derive(Debug, Deserialize)]
pub struct EligibilityQuery {
    pub order_amount: f64,
    pub payment_method: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OrderContext {
    pub order_id: String,
    pub order_amount: f64,
    pub payment_method: Option<String>,
    pub discount_codes: Vec<String>,
}
