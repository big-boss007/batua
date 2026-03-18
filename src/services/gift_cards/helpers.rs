use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::ledger::storage as ledger_storage;
use crate::services::ledger::types::{ActorType, BucketType, MovementType, NewLedgerEntry};
use crate::services::wallets::storage as wallet_storage;
use crate::services::wallets::types::CreateWalletRequest;

use super::storage;
use super::types::{
    BulkIssueRequest, BulkIssueResponse, ClaimGiftCardRequest, GiftCardResponse,
    IssueGiftCardRequest, RedeemGiftCardRequest,
};

const CODE_ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
const CODE_SEGMENT_LEN: usize = 4;
const CODE_SEGMENTS: usize = 3;

#[tracing::instrument]
pub fn generate_gift_card_code() -> String {
    let total_chars = CODE_SEGMENT_LEN * CODE_SEGMENTS;
    let bytes_needed = (total_chars + 15) / 16 * 16;
    let mut raw_bytes = Vec::with_capacity(bytes_needed);

    while raw_bytes.len() < bytes_needed {
        let id = Uuid::new_v4();
        raw_bytes.extend_from_slice(id.as_bytes());
    }

    let mut segments = Vec::with_capacity(CODE_SEGMENTS);
    for seg in 0..CODE_SEGMENTS {
        let mut segment = String::with_capacity(CODE_SEGMENT_LEN);
        for i in 0..CODE_SEGMENT_LEN {
            let byte_idx = seg * CODE_SEGMENT_LEN + i;
            let idx = (raw_bytes[byte_idx] as usize) % CODE_ALPHABET.len();
            segment.push(CODE_ALPHABET[idx] as char);
        }
        segments.push(segment);
    }

    format!("BRZE-{}", segments.join("-"))
}

#[tracing::instrument(err(Debug))]
fn parse_actor_type(s: &str) -> Result<ActorType, AppError> {
    match s {
        "system" | "System" => Ok(ActorType::System),
        "human" | "Human" => Ok(ActorType::Human),
        "automation" | "Automation" => Ok(ActorType::Automation),
        "migration" | "Migration" => Ok(ActorType::Migration),
        other => Err(AppError::BadRequest(format!(
            "unknown actor type: {other}"
        ))),
    }
}

#[tracing::instrument(err(Debug))]
fn parse_expires_at(s: &str) -> Result<DateTime<Utc>, AppError> {
    s.parse::<DateTime<Utc>>()
        .map_err(|e| AppError::BadRequest(format!("invalid expires_at: {e}")))
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn issue_gift_card(
    pool: &PgPool,
    req: IssueGiftCardRequest,
) -> Result<GiftCardResponse, AppError> {
    if req.amount <= 0.0 {
        return Err(AppError::BadRequest(
            "amount must be greater than zero".to_string(),
        ));
    }

    let actor_type = parse_actor_type(&req.actor_type)?;
    let expires_at = match req.expires_at {
        Some(ref s) => Some(parse_expires_at(s)?),
        None => None,
    };

    let code = generate_gift_card_code();

    let bearer_wallet = wallet_storage::create_wallet(
        pool,
        &CreateWalletRequest {
            merchant_id: req.merchant_id,
            customer_id: None,
            is_bearer: true,
            bearer_code: Some(code.clone()),
        },
    )
    .await?;

    let idempotency_key = format!("gc-issue-{}-{}", req.merchant_id, code);
    let ledger_entry = NewLedgerEntry {
        wallet_id: bearer_wallet.id,
        bucket_type: BucketType::GiftCard,
        movement_type: MovementType::In,
        earning_unit: req.amount,
        currency_equivalent: req.amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: actor_type.clone(),
        actor_id: req.actor_id.clone(),
        payment_reference: req.payment_reference.clone(),
        transfer_id: None,
        constraints: serde_json::json!({}),
        expires_at,
    };

    ledger_storage::create_entry(pool, ledger_entry, idempotency_key).await?;

    let gift_card = storage::create_gift_card(
        pool,
        req.merchant_id,
        bearer_wallet.id,
        &code,
        req.amount,
        &actor_type,
        req.actor_id.as_deref(),
        req.payment_reference.as_deref(),
        None,
        None,
        expires_at,
    )
    .await?;

    Ok(gift_card.to_response())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn bulk_issue(
    pool: &PgPool,
    req: BulkIssueRequest,
) -> Result<BulkIssueResponse, AppError> {
    let mut cards = Vec::with_capacity(req.cards.len());
    let mut total_issued: i32 = 0;
    let mut total_skipped: i32 = 0;

    for (position, card_item) in req.cards.iter().enumerate() {
        if card_item.amount <= 0.0 {
            return Err(AppError::BadRequest(format!(
                "card at position {position} has invalid amount: {}",
                card_item.amount
            )));
        }

        let batch_position = position as i32;
        let existing =
            storage::get_gift_card_by_batch_position(pool, req.batch_id, batch_position).await?;

        if let Some(existing_card) = existing {
            total_skipped += 1;
            cards.push(existing_card.to_response());
            continue;
        }

        let code = generate_gift_card_code();

        let bearer_wallet = wallet_storage::create_wallet(
            pool,
            &CreateWalletRequest {
                merchant_id: req.merchant_id,
                customer_id: None,
                is_bearer: true,
                bearer_code: Some(code.clone()),
            },
        )
        .await?;

        let idempotency_key = format!("gc-bulk-{}-{}", req.batch_id, batch_position);
        let ledger_entry = NewLedgerEntry {
            wallet_id: bearer_wallet.id,
            bucket_type: BucketType::GiftCard,
            movement_type: MovementType::In,
            earning_unit: card_item.amount,
            currency_equivalent: card_item.amount,
            conversion_rate: 1.0,
            event_id: None,
            rule_snapshot_id: None,
            campaign_snapshot_id: None,
            actor_type: ActorType::System,
            actor_id: Some(format!("bulk-issue:{}", req.batch_id)),
            payment_reference: None,
            transfer_id: None,
            constraints: serde_json::json!({}),
            expires_at: None,
        };

        ledger_storage::create_entry(pool, ledger_entry, idempotency_key).await?;

        let gift_card = storage::create_gift_card(
            pool,
            req.merchant_id,
            bearer_wallet.id,
            &code,
            card_item.amount,
            &ActorType::System,
            Some(&format!("bulk-issue:{}", req.batch_id)),
            None,
            Some(req.batch_id),
            Some(batch_position),
            None,
        )
        .await?;

        total_issued += 1;
        cards.push(gift_card.to_response());
    }

    Ok(BulkIssueResponse {
        batch_id: req.batch_id,
        total_issued,
        total_skipped,
        cards,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn claim_gift_card(
    pool: &PgPool,
    req: ClaimGiftCardRequest,
) -> Result<GiftCardResponse, AppError> {
    let gift_card = storage::get_gift_card_by_code(pool, &req.code).await?;

    if gift_card.is_claimed {
        return Err(AppError::BadRequest(
            "gift card has already been claimed".to_string(),
        ));
    }

    if !gift_card.is_active {
        return Err(AppError::BadRequest(
            "gift card is not active".to_string(),
        ));
    }

    if let Some(expires_at) = gift_card.expires_at {
        if expires_at < Utc::now() {
            return Err(AppError::BadRequest(
                "gift card has expired".to_string(),
            ));
        }
    }

    let customer_wallet = wallet_storage::get_or_create_wallet(
        pool,
        gift_card.merchant_id,
        req.customer_id,
    )
    .await?;

    let out_entry = NewLedgerEntry {
        wallet_id: gift_card.wallet_id,
        bucket_type: BucketType::GiftCard,
        movement_type: MovementType::Out,
        earning_unit: gift_card.current_amount,
        currency_equivalent: gift_card.current_amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some(format!("gc-claim:{}", gift_card.code)),
        payment_reference: None,
        transfer_id: None,
        constraints: serde_json::json!({}),
        expires_at: gift_card.expires_at,
    };

    let in_entry = NewLedgerEntry {
        wallet_id: customer_wallet.id,
        bucket_type: BucketType::GiftCard,
        movement_type: MovementType::In,
        earning_unit: gift_card.current_amount,
        currency_equivalent: gift_card.current_amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some(format!("gc-claim:{}", gift_card.code)),
        payment_reference: None,
        transfer_id: None,
        constraints: serde_json::json!({}),
        expires_at: gift_card.expires_at,
    };

    let out_key = format!("gc-claim-out-{}", gift_card.id);
    let in_key = format!("gc-claim-in-{}", gift_card.id);

    ledger_storage::create_across_movement(pool, out_entry, out_key, in_entry, in_key).await?;

    let updated = storage::claim_gift_card(pool, gift_card.id, customer_wallet.id).await?;

    Ok(updated.to_response())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn redeem_gift_card(
    pool: &PgPool,
    req: RedeemGiftCardRequest,
) -> Result<GiftCardResponse, AppError> {
    if req.amount <= 0.0 {
        return Err(AppError::BadRequest(
            "redemption amount must be greater than zero".to_string(),
        ));
    }

    let gift_card = storage::get_gift_card_by_code(pool, &req.code).await?;

    if !gift_card.is_active {
        return Err(AppError::BadRequest(
            "gift card is not active".to_string(),
        ));
    }

    if let Some(expires_at) = gift_card.expires_at {
        if expires_at < Utc::now() {
            return Err(AppError::BadRequest(
                "gift card has expired".to_string(),
            ));
        }
    }

    if req.amount > gift_card.current_amount {
        return Err(AppError::BadRequest(format!(
            "insufficient balance: requested {}, available {}",
            req.amount, gift_card.current_amount
        )));
    }

    let redeem_wallet_id = gift_card
        .claimed_by_wallet_id
        .unwrap_or(gift_card.wallet_id);

    let idempotency_key = format!("gc-redeem-{}-{}", gift_card.id, req.order_id);
    let ledger_entry = NewLedgerEntry {
        wallet_id: redeem_wallet_id,
        bucket_type: BucketType::GiftCard,
        movement_type: MovementType::Out,
        earning_unit: req.amount,
        currency_equivalent: req.amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some(format!("gc-redeem:{}", req.order_id)),
        payment_reference: Some(req.order_id.clone()),
        transfer_id: None,
        constraints: serde_json::json!({}),
        expires_at: None,
    };

    ledger_storage::create_entry(pool, ledger_entry, idempotency_key).await?;

    let new_amount = gift_card.current_amount - req.amount;
    let updated = storage::update_gift_card_amount(pool, gift_card.id, new_amount).await?;

    Ok(updated.to_response())
}
