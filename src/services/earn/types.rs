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
