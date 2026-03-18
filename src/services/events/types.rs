use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "event_state", rename_all = "lowercase")]
pub enum EventState {
    Received,
    Processing,
    Processed,
    Failed,
    Duplicate,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Event {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub event_type: String,
    pub event_source: String,
    pub external_event_id: String,
    pub payload: serde_json::Value,
    pub state: EventState,
    pub idempotency_key: String,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct IngestEventRequest {
    pub merchant_id: Uuid,
    pub event_type: String,
    pub event_source: String,
    pub external_event_id: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ShopifyOrderPayload {
    pub id: i64,
    pub order_number: i64,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub total_price: String,
    pub currency: String,
    pub financial_status: String,
    pub gateway: Option<String>,
    pub payment_gateway_names: Option<Vec<String>>,
    pub customer: Option<ShopifyCustomer>,
    pub line_items: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct ShopifyCustomer {
    pub id: i64,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub event_id: Uuid,
    pub state: EventState,
    pub is_duplicate: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListEventsQuery {
    pub merchant_id: Option<Uuid>,
    pub event_type: Option<String>,
    pub state: Option<EventState>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ShopifyWebhookRequest {
    pub merchant_id: Uuid,
    pub payload: serde_json::Value,
}
