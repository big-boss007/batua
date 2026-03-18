use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::error::AppError;

use super::types::ShopifyOrderPayload;

#[tracing::instrument]
pub fn generate_event_idempotency_key(
    merchant_id: Uuid,
    source: &str,
    external_id: &str,
) -> String {
    let input = format!("{merchant_id}:{source}:{external_id}");
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[tracing::instrument(skip(payload), err(Debug))]
pub fn parse_shopify_webhook(payload: &serde_json::Value) -> Result<ShopifyOrderPayload, AppError> {
    serde_json::from_value(payload.clone())
        .map_err(|e| AppError::BadRequest(format!("invalid shopify order payload: {e}")))
}

#[tracing::instrument(skip(payload))]
pub fn extract_payment_method(payload: &ShopifyOrderPayload) -> Option<String> {
    if let Some(ref gateways) = payload.payment_gateway_names {
        if let Some(first) = gateways.first() {
            return Some(first.clone());
        }
    }
    payload.gateway.clone()
}

#[tracing::instrument(skip(payload))]
pub fn is_cod_order(payload: &ShopifyOrderPayload) -> bool {
    let cod_indicators = ["cod", "cash on delivery", "cash_on_delivery"];

    if let Some(ref gateway) = payload.gateway {
        let lower = gateway.to_lowercase();
        if cod_indicators.iter().any(|ind| lower.contains(ind)) {
            return true;
        }
    }

    if let Some(ref gateways) = payload.payment_gateway_names {
        for gw in gateways {
            let lower = gw.to_lowercase();
            if cod_indicators.iter().any(|ind| lower.contains(ind)) {
                return true;
            }
        }
    }

    false
}
