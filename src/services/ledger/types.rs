use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "movement_type", rename_all = "lowercase")]
pub enum MovementType {
    In,
    Held,
    Out,
    Across,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "actor_type", rename_all = "lowercase")]
pub enum ActorType {
    System,
    Human,
    Automation,
    Migration,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "bucket_type", rename_all = "snake_case")]
pub enum BucketType {
    EarnedCredit,
    CodPending,
    GiftCard,
    CustomerFunded,
    ReferralReward,
    GoodwillCredit,
    MembershipBenefit,
    RefundCredit,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "credit_state", rename_all = "lowercase")]
pub enum CreditState {
    Active,
    Expired,
    Redeemed,
    Reversed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct LedgerEntry {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub bucket_type: BucketType,
    pub movement_type: MovementType,
    pub earning_unit: f64,
    pub currency_equivalent: f64,
    pub conversion_rate: f64,
    pub idempotency_key: String,
    pub event_id: Option<Uuid>,
    pub rule_snapshot_id: Option<Uuid>,
    pub campaign_snapshot_id: Option<Uuid>,
    pub actor_type: ActorType,
    pub actor_id: Option<String>,
    pub payment_reference: Option<String>,
    pub transfer_id: Option<Uuid>,
    pub constraints: serde_json::Value,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub state: CreditState,
}

#[derive(Debug, Deserialize)]
pub struct NewLedgerEntry {
    pub wallet_id: Uuid,
    pub bucket_type: BucketType,
    pub movement_type: MovementType,
    pub earning_unit: f64,
    pub currency_equivalent: f64,
    pub conversion_rate: f64,
    pub event_id: Option<Uuid>,
    pub rule_snapshot_id: Option<Uuid>,
    pub campaign_snapshot_id: Option<Uuid>,
    pub actor_type: ActorType,
    pub actor_id: Option<String>,
    pub payment_reference: Option<String>,
    pub transfer_id: Option<Uuid>,
    pub constraints: serde_json::Value,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct WalletBalance {
    pub wallet_id: Uuid,
    pub displayed_balance: f64,
    pub spendable_balance: f64,
    pub buckets: Vec<BucketBalance>,
}

#[derive(Debug, Serialize)]
pub struct BucketBalance {
    pub bucket_type: BucketType,
    pub displayed: f64,
    pub spendable: f64,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub wallet_id: Uuid,
    pub bucket_type: BucketType,
    pub movement_type: MovementType,
    pub earning_unit: f64,
    pub currency_equivalent: f64,
    pub conversion_rate: f64,
    pub event_id: Option<Uuid>,
    pub rule_snapshot_id: Option<Uuid>,
    pub campaign_snapshot_id: Option<Uuid>,
    pub actor_type: ActorType,
    pub actor_id: Option<String>,
    pub payment_reference: Option<String>,
    pub transfer_id: Option<Uuid>,
    pub constraints: Option<serde_json::Value>,
    pub expires_at: Option<DateTime<Utc>>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetEntriesQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub bucket_type: Option<BucketType>,
    pub movement_type: Option<MovementType>,
}

#[derive(Debug, Deserialize)]
pub struct BalanceAtQuery {
    pub at: DateTime<Utc>,
}
