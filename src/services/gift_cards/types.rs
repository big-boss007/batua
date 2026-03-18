use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GiftCard {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub wallet_id: Uuid,
    pub code: String,
    pub initial_amount: f64,
    pub current_amount: f64,
    pub currency: String,
    pub issued_by: crate::services::ledger::types::ActorType,
    pub issued_by_id: Option<String>,
    pub payment_reference: Option<String>,
    pub batch_id: Option<Uuid>,
    pub batch_position: Option<i32>,
    pub is_claimed: bool,
    pub claimed_by_wallet_id: Option<Uuid>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct IssueGiftCardRequest {
    pub merchant_id: Uuid,
    pub amount: f64,
    pub expires_at: Option<String>,
    pub payment_reference: Option<String>,
    pub actor_type: String,
    pub actor_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkIssueRequest {
    pub merchant_id: Uuid,
    pub batch_id: Uuid,
    pub cards: Vec<BulkCardItem>,
}

#[derive(Debug, Deserialize)]
pub struct BulkCardItem {
    pub amount: f64,
    pub recipient_phone: Option<String>,
    pub recipient_email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClaimGiftCardRequest {
    pub code: String,
    pub customer_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct RedeemGiftCardRequest {
    pub code: String,
    pub amount: f64,
    pub order_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferGiftCardRequest {
    pub code: String,
    pub from_wallet_id: Option<Uuid>,
    pub to_customer_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct GiftCardResponse {
    pub id: Uuid,
    pub code: String,
    pub initial_amount: f64,
    pub current_amount: f64,
    pub is_claimed: bool,
    pub is_active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BulkIssueResponse {
    pub batch_id: Uuid,
    pub total_issued: i32,
    pub total_skipped: i32,
    pub cards: Vec<GiftCardResponse>,
}

impl GiftCard {
    pub fn to_response(&self) -> GiftCardResponse {
        GiftCardResponse {
            id: self.id,
            code: self.code.clone(),
            initial_amount: self.initial_amount,
            current_amount: self.current_amount,
            is_claimed: self.is_claimed,
            is_active: self.is_active,
            expires_at: self.expires_at,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct GiftCardStats {
    pub total_issued: i64,
    pub total_outstanding_value: f64,
    pub total_redeemed_value: f64,
    pub total_expired: i64,
    pub total_active: i64,
    pub total_claimed: i64,
}
