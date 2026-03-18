use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::ledger;
use crate::services::ledger::types::{ActorType, BucketType, MovementType, NewLedgerEntry};
use crate::services::wallets;

use super::storage;
use super::types::{FraudCheckResult, ProcessReferralRequest, ReferralCode, ReferralResponse};

const REFERRAL_CODE_LENGTH: usize = 8;
const ALPHANUMERIC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_referral(
    pool: &PgPool,
    req: &ProcessReferralRequest,
) -> Result<ReferralResponse, AppError> {
    let referral_code = storage::get_referral_code(pool, &req.referral_code).await?;

    if !referral_code.is_active {
        return Err(AppError::BadRequest(
            "referral code is no longer active".to_string(),
        ));
    }

    if referral_code.merchant_id != req.merchant_id {
        return Err(AppError::BadRequest(
            "referral code does not belong to this merchant".to_string(),
        ));
    }

    let fraud_result = check_fraud(pool, req, &referral_code).await?;

    let program = storage::get_program(pool, req.merchant_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound("no referral program found for this merchant".to_string())
        })?;

    if !program.is_active {
        return Err(AppError::BadRequest(
            "referral program is not active".to_string(),
        ));
    }

    if let Some(max) = program.max_referrals_per_customer {
        let count =
            storage::count_customer_referrals(pool, req.merchant_id, referral_code.customer_id)
                .await?;
        if count >= max as i64 {
            return Err(AppError::BadRequest(format!(
                "referrer has reached the maximum of {max} referrals"
            )));
        }
    }

    let mut referrer_rewarded = false;
    let mut referee_rewarded = false;
    let mut referrer_entry_id: Option<Uuid> = None;
    let mut referee_entry_id: Option<Uuid> = None;

    if !fraud_result.is_suspicious {
        let order_part = req
            .order_id
            .as_deref()
            .unwrap_or("no_order");

        let referrer_wallet = wallets::storage::get_or_create_wallet(
            pool,
            req.merchant_id,
            referral_code.customer_id,
        )
        .await?;

        let referrer_idempotency_key = format!(
            "referral_referrer_{}_{}_{}_{}",
            referral_code.code, req.referee_id, order_part, referrer_wallet.id
        );

        let referrer_entry = ledger::storage::create_entry(
            pool,
            NewLedgerEntry {
                wallet_id: referrer_wallet.id,
                bucket_type: BucketType::ReferralReward,
                movement_type: MovementType::In,
                earning_unit: program.referrer_reward_amount,
                currency_equivalent: program.referrer_reward_amount,
                conversion_rate: 1.0,
                event_id: None,
                rule_snapshot_id: None,
                campaign_snapshot_id: None,
                actor_type: ActorType::System,
                actor_id: Some("referral_service".to_string()),
                payment_reference: req.order_id.clone(),
                transfer_id: None,
                constraints: serde_json::json!({}),
                expires_at: None,
            },
            referrer_idempotency_key,
        )
        .await?;

        referrer_entry_id = Some(referrer_entry.id);
        referrer_rewarded = true;

        let referee_wallet =
            wallets::storage::get_or_create_wallet(pool, req.merchant_id, req.referee_id).await?;

        let referee_idempotency_key = format!(
            "referral_referee_{}_{}_{}_{}",
            referral_code.code, req.referee_id, order_part, referee_wallet.id
        );

        let referee_entry = ledger::storage::create_entry(
            pool,
            NewLedgerEntry {
                wallet_id: referee_wallet.id,
                bucket_type: BucketType::ReferralReward,
                movement_type: MovementType::In,
                earning_unit: program.referee_reward_amount,
                currency_equivalent: program.referee_reward_amount,
                conversion_rate: 1.0,
                event_id: None,
                rule_snapshot_id: None,
                campaign_snapshot_id: None,
                actor_type: ActorType::System,
                actor_id: Some("referral_service".to_string()),
                payment_reference: req.order_id.clone(),
                transfer_id: None,
                constraints: serde_json::json!({}),
                expires_at: None,
            },
            referee_idempotency_key,
        )
        .await?;

        referee_entry_id = Some(referee_entry.id);
        referee_rewarded = true;
    }

    let fraud_signals_json = serde_json::to_value(&fraud_result.signals)
        .unwrap_or_else(|_| serde_json::json!([]));

    let conversion = storage::create_conversion(
        pool,
        req.merchant_id,
        referral_code.id,
        referral_code.customer_id,
        req.referee_id,
        req.order_id.as_deref(),
        referrer_entry_id,
        referee_entry_id,
        req.referee_ip.as_deref(),
        req.referee_device_fingerprint.as_deref(),
        fraud_result.is_suspicious,
        &fraud_signals_json,
    )
    .await?;

    storage::increment_referral_stats(pool, referral_code.id, !fraud_result.is_suspicious).await?;

    Ok(ReferralResponse {
        conversion_id: conversion.id,
        referrer_rewarded,
        referee_rewarded,
        fraud_signals: fraud_result.signals,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn check_fraud(
    pool: &PgPool,
    req: &ProcessReferralRequest,
    referral_code: &ReferralCode,
) -> Result<FraudCheckResult, AppError> {
    let mut signals: Vec<String> = Vec::new();

    if referral_code.customer_id == req.referee_id {
        signals.push("self_referral".to_string());
    }

    if let Some(ref referee_ip) = req.referee_ip {
        let same_ip_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)::int8
            FROM referral_conversions
            WHERE referral_code_id = $1 AND referee_ip = $2
            "#,
        )
        .bind(referral_code.id)
        .bind(referee_ip)
        .fetch_one(pool)
        .await?;

        if same_ip_count.0 > 0 {
            signals.push("duplicate_ip".to_string());
        }
    }

    if let Some(ref fingerprint) = req.referee_device_fingerprint {
        let same_device_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)::int8
            FROM referral_conversions
            WHERE referral_code_id = $1 AND referee_device_fingerprint = $2
            "#,
        )
        .bind(referral_code.id)
        .bind(fingerprint)
        .fetch_one(pool)
        .await?;

        if same_device_count.0 > 0 {
            signals.push("duplicate_device_fingerprint".to_string());
        }
    }

    let recent_referrals: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)::int8
        FROM referral_conversions
        WHERE referral_code_id = $1 AND created_at > now() - INTERVAL '1 hour'
        "#,
    )
    .bind(referral_code.id)
    .fetch_one(pool)
    .await?;

    if recent_referrals.0 >= 10 {
        signals.push("high_velocity".to_string());
    }

    let referee_age: Option<(bool,)> = sqlx::query_as(
        r#"
        SELECT (created_at > now() - INTERVAL '5 minutes') AS is_recent
        FROM customers
        WHERE id = $1
        "#,
    )
    .bind(req.referee_id)
    .fetch_optional(pool)
    .await?;

    if let Some((true,)) = referee_age {
        signals.push("new_account_referee".to_string());
    }

    let is_suspicious = !signals.is_empty();

    Ok(FraudCheckResult {
        is_suspicious,
        signals,
    })
}

#[tracing::instrument]
pub fn generate_referral_code(customer_name: Option<&str>) -> String {
    if let Some(name) = customer_name {
        let clean: String = name
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .take(4)
            .collect::<String>()
            .to_uppercase();

        if clean.len() >= 2 {
            let id = Uuid::new_v4();
            let bytes = id.as_bytes();
            let d1 = (bytes[0] as u16 % 10) as u8;
            let d2 = (bytes[1] as u16 % 10) as u8;
            return format!("{clean}{d1}{d2}");
        }
    }

    generate_random_code()
}

fn generate_random_code() -> String {
    let mut code = String::with_capacity(REFERRAL_CODE_LENGTH);
    let bytes_needed = (REFERRAL_CODE_LENGTH + 15) / 16 * 16;
    let mut raw_bytes = Vec::with_capacity(bytes_needed);

    while raw_bytes.len() < bytes_needed {
        let id = Uuid::new_v4();
        raw_bytes.extend_from_slice(id.as_bytes());
    }

    for &byte in raw_bytes.iter().take(REFERRAL_CODE_LENGTH) {
        let idx = (byte as usize) % ALPHANUMERIC.len();
        code.push(ALPHANUMERIC[idx] as char);
    }

    code
}
