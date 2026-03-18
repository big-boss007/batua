use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Wallet {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub is_bearer: bool,
    pub bearer_code: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub merchant_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub is_bearer: bool,
    pub bearer_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WalletResponse {
    pub wallet: Wallet,
    pub balance: Option<WalletBalanceSummary>,
}

#[derive(Debug, Serialize)]
pub struct WalletBalanceSummary {
    pub displayed_balance: f64,
    pub spendable_balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct WalletLookupQuery {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct GetOrCreateRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}
