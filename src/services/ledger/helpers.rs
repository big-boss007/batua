use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::types::{LedgerEntry, MovementType};

#[tracing::instrument]
pub fn generate_idempotency_key(
    merchant_id: &str,
    event_id: Option<Uuid>,
    rule_id: Option<Uuid>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(merchant_id.as_bytes());
    if let Some(eid) = event_id {
        hasher.update(eid.as_bytes());
    }
    if let Some(rid) = rule_id {
        hasher.update(rid.as_bytes());
    }
    hex::encode(hasher.finalize())
}

#[tracing::instrument]
pub fn validate_double_entry(entries: &[LedgerEntry]) -> bool {
    let mut total_in: f64 = 0.0;
    let mut total_out: f64 = 0.0;

    for entry in entries {
        match entry.movement_type {
            MovementType::In | MovementType::Held => {
                total_in += entry.earning_unit;
            }
            MovementType::Out => {
                total_out += entry.earning_unit;
            }
            MovementType::Across => {}
        }
    }

    (total_in - total_out).abs() < f64::EPSILON
}
