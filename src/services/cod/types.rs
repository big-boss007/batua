use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CodOrderState {
    Pending,
    Delivered,
    Rto,
    Cancelled,
}

impl CodOrderState {
    pub fn as_str(&self) -> &'static str {
        match self {
            CodOrderState::Pending => "pending",
            CodOrderState::Delivered => "delivered",
            CodOrderState::Rto => "rto",
            CodOrderState::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(CodOrderState::Pending),
            "delivered" => Ok(CodOrderState::Delivered),
            "rto" => Ok(CodOrderState::Rto),
            "cancelled" => Ok(CodOrderState::Cancelled),
            other => Err(format!("unknown COD order state: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CodOrder {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub order_id: String,
    pub wallet_id: Uuid,
    pub ledger_entry_id: Uuid,
    pub state: String,
    pub delivery_confirmed_at: Option<DateTime<Utc>>,
    pub released_entry_id: Option<Uuid>,
    pub cancelled_entry_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct DeliveryWebhookPayload {
    pub order_id: String,
    pub status: String,
    pub delivered_at: Option<String>,
    pub merchant_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CodToPrepaidRequest {
    pub merchant_id: Uuid,
    pub order_id: String,
    pub customer_id: Uuid,
    pub order_amount: f64,
    pub new_payment_method: String,
}

#[derive(Debug, Serialize)]
pub struct CodToPrepaidResponse {
    pub incentive_amount: f64,
    pub ledger_entry_id: Uuid,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct CodOrderResponse {
    pub id: Uuid,
    pub order_id: String,
    pub state: String,
    pub pending_amount: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CodAnalytics {
    pub total_pending: i64,
    pub total_delivered: i64,
    pub total_rto: i64,
    pub pending_amount: f64,
    pub released_amount: f64,
    pub cancelled_amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct CodOrdersQuery {
    pub state: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}
