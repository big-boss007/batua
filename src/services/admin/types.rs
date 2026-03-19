use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Merchant {
    pub id: Uuid,
    pub external_id: String,
    pub name: String,
    pub domain: Option<String>,
    pub currency: String,
    pub timezone: String,
    pub is_active: bool,
    pub slug: Option<String>,
    pub geo_policy_id: Option<Uuid>,
    pub plan_tier: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMerchantRequest {
    pub external_id: String,
    pub name: String,
    pub domain: Option<String>,
    pub currency: Option<String>,
    pub timezone: Option<String>,
    pub slug: Option<String>,
    pub plan_tier: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMerchantRequest {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub is_active: Option<bool>,
    pub slug: Option<String>,
    pub plan_tier: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkCreditRequest {
    pub merchant_id: Uuid,
    pub customer_ids: Vec<Uuid>,
    pub amount: f64,
    pub bucket_type: String,
    pub reason: String,
    pub actor_id: String,
}

#[derive(Debug, Serialize)]
pub struct BulkCreditResult {
    pub total_processed: i32,
    pub total_succeeded: i32,
    pub total_failed: i32,
    pub results: Vec<BulkCreditItemResult>,
}

#[derive(Debug, Serialize)]
pub struct BulkCreditItemResult {
    pub customer_id: Uuid,
    pub success: bool,
    pub ledger_entry_id: Option<Uuid>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DisputeRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    pub ledger_entry_id: Uuid,
    pub reason: String,
    pub actor_id: String,
}

#[derive(Debug, Serialize)]
pub struct DisputeResult {
    pub reversal_entry_id: Uuid,
    pub original_amount: f64,
    pub reversed: bool,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GeoPolicy {
    pub id: Uuid,
    pub geo_code: String,
    pub name: String,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct WalletPolicyRequest {
    pub merchant_id: Uuid,
    pub bucket_type: String,
    pub min_redemption: Option<f64>,
    pub step_size: Option<f64>,
    pub max_per_order_pct: Option<f64>,
    pub max_per_order_fixed: Option<f64>,
    pub stackable_with_discounts: Option<bool>,
    pub default_expiry_days: Option<i32>,
    pub is_transferable: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AdminDashboard {
    pub total_merchants: i64,
    pub total_wallets: i64,
    pub total_ledger_entries: i64,
    pub total_value_in_system: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateGeoPolicyRequest {
    pub geo_code: String,
    pub name: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct MerchantStats {
    pub merchant_id: Uuid,
    pub total_wallets: i64,
    pub total_customers: i64,
    pub total_ledger_entries: i64,
    pub active_credits: f64,
    pub total_redeemed: f64,
}

#[derive(Debug, Serialize)]
pub struct SystemHealth {
    pub unprocessed_events: i64,
    pub failed_events: i64,
    pub pending_cod_orders: i64,
    pub expiring_7d_count: i64,
    pub expiring_7d_value: f64,
    pub expiring_30d_count: i64,
    pub expiring_30d_value: f64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct RecentEvent {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub merchant_name: String,
    pub event_type: String,
    pub event_source: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlanRequest {
    pub plan_tier: String,
}

#[derive(Debug, Deserialize)]
pub struct RecentEventsQuery {
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct MerchantDashboard {
    pub merchant_id: Uuid,
    pub active_customers: i64,
    pub total_wallets: i64,
    pub total_earned: f64,
    pub total_redeemed: f64,
    pub total_cod_pending: f64,
    pub active_credits: f64,
    pub total_ledger_entries: i64,
    pub redemption_count: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MerchantCustomer {
    pub customer_id: Uuid,
    pub customer_name: Option<String>,
    pub customer_phone: String,
    pub customer_email: Option<String>,
    pub wallet_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct MerchantCustomersQuery {
    pub search: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MerchantTransaction {
    pub entry_id: Uuid,
    pub wallet_id: Uuid,
    pub customer_name: Option<String>,
    pub customer_phone: String,
    pub bucket_type: String,
    pub movement_type: String,
    pub currency_equivalent: f64,
    pub state: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct MerchantTransactionsQuery {
    pub search: Option<String>,
    pub bucket_type: Option<String>,
    pub movement_type: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct MerchantAnalytics {
    pub total_earned: f64,
    pub total_redeemed: f64,
    pub total_expired: f64,
    pub active_credits: f64,
    pub cod_pending: f64,
    pub cod_delivered: f64,
    pub cod_rto: f64,
    pub total_orders: i64,
    pub prepaid_orders: i64,
    pub cod_orders: i64,
    pub loyalty_rto_rate: f64,
    pub non_loyalty_rto_rate: f64,
    pub repeat_purchase_rate: f64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Coalition {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CoalitionMember {
    pub id: Uuid,
    pub coalition_id: Uuid,
    pub merchant_id: Uuid,
    pub conversion_rate: f64,
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCoalitionRequest {
    pub name: String,
    pub merchant_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CoalitionTransferRequest {
    pub customer_id: Uuid,
    pub from_merchant_id: Uuid,
    pub to_merchant_id: Uuid,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct CoalitionTransferResult {
    pub transfer_id: Uuid,
    pub from_amount: f64,
    pub to_amount: f64,
    pub conversion_rate: f64,
    pub from_balance_after: f64,
    pub to_balance_after: f64,
}

#[derive(Debug, Serialize)]
pub struct CoalitionInfo {
    pub coalition: Coalition,
    pub members: Vec<CoalitionMemberInfo>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CoalitionMemberInfo {
    pub merchant_id: Uuid,
    pub merchant_name: String,
    pub conversion_rate: f64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CoalitionTransferRecord {
    pub id: Uuid,
    pub coalition_id: Uuid,
    pub customer_id: Uuid,
    pub from_merchant_id: Uuid,
    pub to_merchant_id: Uuid,
    pub from_wallet_id: Uuid,
    pub to_wallet_id: Uuid,
    pub amount: f64,
    pub converted_amount: f64,
    pub conversion_rate: f64,
    pub transfer_id: Uuid,
    pub created_at: DateTime<Utc>,
}
