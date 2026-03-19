use chrono::{Days, Utc};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::events;
use crate::services::events::types::{EventState, ShopifyOrderPayload};
use crate::services::identity;
use crate::services::identity::types::ResolveIdentityRequest;
use crate::services::ledger;
use crate::services::ledger::types::{ActorType, BucketType, MovementType, NewLedgerEntry};
use crate::services::rules;
use crate::services::rules::types::EvaluationContext;
use crate::services::cod;
use crate::services::wallets;

use super::storage;
use super::types::{
    BirthdayBonusEntry, BirthdayBonusResult, EarnEntry, EarnResult, ManualCreditRequest,
    ManualCreditResult, MilestoneAchievementEntry, MilestoneCheckResult,
    NewsletterSignupRequest, NewsletterSignupResult, ProfileCompletionRequest,
    ProfileCompletionResult,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_earn(pool: &PgPool, event_id: Uuid) -> Result<EarnResult, AppError> {
    let event = events::storage::get_event(pool, event_id).await?;

    if event.state != EventState::Received {
        return Err(AppError::BadRequest(format!(
            "event {} is in state {:?}, expected Received",
            event_id, event.state
        )));
    }

    events::storage::mark_event_state(pool, event_id, EventState::Processing, None).await?;

    let result = do_process_earn(pool, &event).await;

    match &result {
        Ok(_) => {
            events::storage::mark_event_state(
                pool,
                event_id,
                EventState::Processed,
                Some(Utc::now()),
            )
            .await?;
        }
        Err(_) => {
            events::storage::mark_event_state(pool, event_id, EventState::Failed, None).await?;
        }
    }

    result
}

#[tracing::instrument(skip(pool, event), err(Debug))]
async fn do_process_earn(
    pool: &PgPool,
    event: &events::types::Event,
) -> Result<EarnResult, AppError> {
    let order_payload = parse_order_payload(&event.payload)?;

    let customer_phone = extract_customer_phone(&order_payload)?;
    let customer_email = extract_customer_email(&order_payload);
    let customer_name = extract_customer_name(&order_payload);

    let order_amount = order_payload
        .total_price
        .parse::<f64>()
        .map_err(|e| AppError::BadRequest(format!("invalid total_price: {e}")))?;

    let is_cod = events::helpers::is_cod_order(&order_payload);
    let payment_method = events::helpers::extract_payment_method(&order_payload);

    let resolve_req = ResolveIdentityRequest {
        phone: customer_phone,
        email: customer_email,
        name: customer_name,
        external_id: None,
    };
    let (customer, _is_new) = identity::storage::resolve_or_create(pool, &resolve_req).await?;

    let wallet =
        wallets::storage::get_or_create_wallet(pool, event.merchant_id, customer.id).await?;

    let order_stats =
        storage::get_customer_order_stats(pool, event.merchant_id, customer.id).await?;
    let is_first_order = order_stats.is_none();

    let context = build_evaluation_context(event, order_amount, payment_method, is_cod, is_first_order);

    let eval_results = rules::helpers::evaluate_rules(pool, &context).await?;

    let mut entries_created = Vec::new();

    for eval in &eval_results {
        if !eval.matched || eval.earning_unit <= 0.0 {
            continue;
        }

        let (movement_type, bucket_type) = if is_cod {
            (MovementType::Held, BucketType::CodPending)
        } else {
            let bt = parse_bucket_type(&eval.bucket_type)?;
            (MovementType::In, bt)
        };

        let rule_snapshot_id = eval.rule_snapshot_id;
        let idempotency_key = generate_earn_idempotency_key(
            event.id,
            rule_snapshot_id,
        );

        let expires_at = eval.expiry_days.and_then(|days| {
            let d = days as u64;
            Utc::now().checked_add_days(Days::new(d))
        });

        let new_entry = NewLedgerEntry {
            wallet_id: wallet.id,
            bucket_type: bucket_type.clone(),
            movement_type: movement_type.clone(),
            earning_unit: eval.earning_unit,
            currency_equivalent: eval.currency_equivalent,
            conversion_rate: eval.conversion_rate,
            event_id: Some(event.id),
            rule_snapshot_id: eval.rule_snapshot_id,
            campaign_snapshot_id: eval.campaign_snapshot_id,
            actor_type: ActorType::System,
            actor_id: None,
            payment_reference: Some(format!("order:{}", event.external_event_id)),
            transfer_id: None,
            constraints: eval.constraints.clone(),
            expires_at,
        };

        let entry =
            ledger::storage::create_entry(pool, new_entry, idempotency_key).await?;

        if is_cod {
            let _ = cod::storage::create_cod_order(
                pool,
                event.merchant_id,
                &event.external_event_id,
                wallet.id,
                entry.id,
            )
            .await;
        }

        entries_created.push(EarnEntry {
            ledger_entry_id: entry.id,
            bucket_type: format!("{:?}", bucket_type),
            earning_unit: entry.earning_unit,
            currency_equivalent: entry.currency_equivalent,
            movement_type: format!("{:?}", movement_type),
        });
    }

    storage::update_order_stats(pool, event.merchant_id, customer.id, order_amount).await?;

    Ok(EarnResult {
        event_id: event.id,
        customer_id: customer.id,
        wallet_id: wallet.id,
        entries_created,
        is_cod,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_manual_credit(
    pool: &PgPool,
    req: ManualCreditRequest,
) -> Result<ManualCreditResult, AppError> {
    let wallet =
        wallets::storage::get_or_create_wallet(pool, req.merchant_id, req.customer_id).await?;

    let bucket_type = parse_bucket_type(&req.bucket_type).unwrap_or(BucketType::GoodwillCredit);

    let idempotency_key = format!(
        "manual:{}:{}:{}",
        req.merchant_id,
        req.customer_id,
        Uuid::new_v4()
    );

    let new_entry = NewLedgerEntry {
        wallet_id: wallet.id,
        bucket_type,
        movement_type: MovementType::In,
        earning_unit: req.amount,
        currency_equivalent: req.amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::Human,
        actor_id: Some(req.actor_id),
        payment_reference: Some(req.reason),
        transfer_id: None,
        constraints: serde_json::json!({}),
        expires_at: None,
    };

    let entry = ledger::storage::create_entry(pool, new_entry, idempotency_key).await?;

    Ok(ManualCreditResult {
        ledger_entry_id: entry.id,
        wallet_id: wallet.id,
        amount: entry.earning_unit,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_birthday_bonuses(
    pool: &PgPool,
    merchant_id: Uuid,
    amount: f64,
) -> Result<BirthdayBonusResult, AppError> {
    let birthday_customers =
        identity::storage::get_customers_with_birthday_today(pool, merchant_id).await?;

    let processed = birthday_customers.len() as i32;
    let mut credited = 0i32;
    let mut skipped = 0i32;
    let mut entries = Vec::new();

    let today = Utc::now().format("%Y-%m-%d").to_string();

    for cw in &birthday_customers {
        let hash_input = format!("{}{}birthday{}", merchant_id, cw.id, today);
        let mut hasher = Sha256::new();
        hasher.update(hash_input.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        let idempotency_key = format!("birthday:{hash}");

        let new_entry = NewLedgerEntry {
            wallet_id: cw.wallet_id,
            bucket_type: BucketType::EarnedCredit,
            movement_type: MovementType::In,
            earning_unit: amount,
            currency_equivalent: amount,
            conversion_rate: 1.0,
            event_id: None,
            rule_snapshot_id: None,
            campaign_snapshot_id: None,
            actor_type: ActorType::System,
            actor_id: Some("birthday_bonus".to_string()),
            payment_reference: Some(format!("birthday_bonus:{}", today)),
            transfer_id: None,
            constraints: serde_json::json!({}),
            expires_at: None,
        };

        let entry = ledger::storage::create_entry(pool, new_entry, idempotency_key).await?;

        if entry.earning_unit > 0.0 {
            credited += 1;
            entries.push(BirthdayBonusEntry {
                customer_id: cw.id,
                customer_name: cw.name.clone(),
                amount: entry.earning_unit,
                ledger_entry_id: entry.id,
            });
        } else {
            skipped += 1;
        }
    }

    Ok(BirthdayBonusResult {
        merchant_id,
        processed,
        credited,
        skipped,
        entries,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_newsletter_signup(
    pool: &PgPool,
    req: NewsletterSignupRequest,
) -> Result<NewsletterSignupResult, AppError> {
    if !identity::helpers::validate_email(&req.email) {
        return Err(AppError::BadRequest(format!(
            "invalid email: {}",
            req.email
        )));
    }

    let customer_id = resolve_newsletter_customer(pool, &req).await?;

    let already = storage::has_newsletter_signup(pool, req.merchant_id, customer_id).await?;
    if already {
        return Ok(NewsletterSignupResult {
            customer_id,
            email: req.email,
            rewarded: false,
            already_subscribed: true,
            ledger_entry_id: None,
            amount: 0.0,
        });
    }

    let wallet =
        wallets::storage::get_or_create_wallet(pool, req.merchant_id, customer_id).await?;

    let hash_input = format!("{}{}newsletter_signup", req.merchant_id, customer_id);
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    let idempotency_key = format!("newsletter:{hash}");

    let new_entry = NewLedgerEntry {
        wallet_id: wallet.id,
        bucket_type: BucketType::EarnedCredit,
        movement_type: MovementType::In,
        earning_unit: req.amount,
        currency_equivalent: req.amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some("newsletter_signup".to_string()),
        payment_reference: Some(format!("newsletter_signup:{}", req.email)),
        transfer_id: None,
        constraints: serde_json::json!({}),
        expires_at: None,
    };

    let entry = ledger::storage::create_entry(pool, new_entry, idempotency_key).await?;

    storage::record_newsletter_signup(
        pool,
        req.merchant_id,
        customer_id,
        entry.id,
        &req.email,
        "webhook",
    )
    .await?;

    Ok(NewsletterSignupResult {
        customer_id,
        email: req.email,
        rewarded: true,
        already_subscribed: false,
        ledger_entry_id: Some(entry.id),
        amount: entry.earning_unit,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
async fn resolve_newsletter_customer(
    pool: &PgPool,
    req: &NewsletterSignupRequest,
) -> Result<Uuid, AppError> {
    if let Some(customer_id) = req.customer_id {
        let _customer = identity::storage::get_customer(pool, customer_id).await?;
        return Ok(customer_id);
    }

    if let Some(ref phone) = req.phone {
        let resolve_req = ResolveIdentityRequest {
            phone: phone.clone(),
            email: Some(req.email.clone()),
            name: None,
            external_id: None,
        };
        let (customer, _is_new) = identity::storage::resolve_or_create(pool, &resolve_req).await?;
        return Ok(customer.id);
    }

    if let Some(customer) = identity::storage::resolve_by_email(pool, &req.email).await? {
        return Ok(customer.id);
    }

    Err(AppError::BadRequest(
        "cannot resolve customer: provide customer_id or phone, or ensure a customer with this email exists".to_string(),
    ))
}

#[tracing::instrument(skip(event))]
fn build_evaluation_context(
    event: &events::types::Event,
    order_amount: f64,
    payment_method: Option<String>,
    is_cod: bool,
    is_first_order: bool,
) -> EvaluationContext {
    EvaluationContext {
        merchant_id: event.merchant_id,
        event_type: event.event_type.clone(),
        event_payload: event.payload.clone(),
        order_amount: Some(order_amount),
        payment_method,
        is_cod,
        collections: Vec::new(),
        customer_tags: Vec::new(),
        is_first_order,
    }
}

#[tracing::instrument(skip(payload), err(Debug))]
fn parse_order_payload(
    payload: &serde_json::Value,
) -> Result<ShopifyOrderPayload, AppError> {
    serde_json::from_value(payload.clone())
        .map_err(|e| AppError::BadRequest(format!("invalid order payload: {e}")))
}

#[tracing::instrument(skip(payload), err(Debug))]
fn extract_customer_phone(payload: &ShopifyOrderPayload) -> Result<String, AppError> {
    if let Some(ref customer) = payload.customer {
        if let Some(ref phone) = customer.phone {
            if !phone.is_empty() {
                return Ok(phone.clone());
            }
        }
    }

    if let Some(ref phone) = payload.phone {
        if !phone.is_empty() {
            return Ok(phone.clone());
        }
    }

    Err(AppError::BadRequest(
        "no customer phone found in order payload".to_string(),
    ))
}

#[tracing::instrument(skip(payload))]
fn extract_customer_email(payload: &ShopifyOrderPayload) -> Option<String> {
    if let Some(ref customer) = payload.customer {
        if let Some(ref email) = customer.email {
            if !email.is_empty() {
                return Some(email.clone());
            }
        }
    }

    payload.email.clone().filter(|e| !e.is_empty())
}

#[tracing::instrument(skip(payload))]
fn extract_customer_name(payload: &ShopifyOrderPayload) -> Option<String> {
    if let Some(ref customer) = payload.customer {
        let first = customer.first_name.as_deref().unwrap_or("");
        let last = customer.last_name.as_deref().unwrap_or("");
        let full_name = format!("{first} {last}").trim().to_string();
        if !full_name.is_empty() {
            return Some(full_name);
        }
    }
    None
}

fn generate_earn_idempotency_key(event_id: Uuid, rule_snapshot_id: Option<Uuid>) -> String {
    match rule_snapshot_id {
        Some(rsid) => format!("earn:{event_id}:{rsid}"),
        None => format!("earn:{event_id}"),
    }
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn process_profile_completion(
    pool: &PgPool,
    req: ProfileCompletionRequest,
) -> Result<ProfileCompletionResult, AppError> {
    let customer = identity::storage::get_customer(pool, req.customer_id).await?;

    let name_filled = customer.name.as_ref().map_or(false, |n| !n.is_empty());
    let email_filled = customer.email.as_ref().map_or(false, |e| !e.is_empty());
    let birthday_filled = customer.birthday.is_some();

    let mut fields_complete = Vec::new();
    let mut fields_missing = Vec::new();

    if name_filled {
        fields_complete.push("name".to_string());
    } else {
        fields_missing.push("name".to_string());
    }
    if email_filled {
        fields_complete.push("email".to_string());
    } else {
        fields_missing.push("email".to_string());
    }
    if birthday_filled {
        fields_complete.push("birthday".to_string());
    } else {
        fields_missing.push("birthday".to_string());
    }

    let total_fields = 3.0_f64;
    let completion_pct = (fields_complete.len() as f64 / total_fields) * 100.0;

    if !fields_missing.is_empty() {
        return Ok(ProfileCompletionResult {
            customer_id: req.customer_id,
            fields_complete,
            fields_missing,
            completion_pct,
            already_rewarded: false,
            rewarded: false,
            amount: 0.0,
            ledger_entry_id: None,
        });
    }

    let hash_input = format!("{}{}profile_complete", req.merchant_id, req.customer_id);
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    let idempotency_key = format!("profile_complete:{hash}");

    let already_exists =
        ledger::storage::entry_exists_by_idempotency_key(pool, &idempotency_key).await?;

    if already_exists {
        return Ok(ProfileCompletionResult {
            customer_id: req.customer_id,
            fields_complete,
            fields_missing,
            completion_pct,
            already_rewarded: true,
            rewarded: false,
            amount: 0.0,
            ledger_entry_id: None,
        });
    }

    let wallet =
        wallets::storage::get_or_create_wallet(pool, req.merchant_id, req.customer_id).await?;

    let amount = 30.0_f64;

    let new_entry = NewLedgerEntry {
        wallet_id: wallet.id,
        bucket_type: BucketType::EarnedCredit,
        movement_type: MovementType::In,
        earning_unit: amount,
        currency_equivalent: amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some("profile_completion".to_string()),
        payment_reference: Some("profile_completion:all_fields".to_string()),
        transfer_id: None,
        constraints: serde_json::json!({}),
        expires_at: None,
    };

    let entry = ledger::storage::create_entry(pool, new_entry, idempotency_key).await?;

    Ok(ProfileCompletionResult {
        customer_id: req.customer_id,
        fields_complete,
        fields_missing,
        completion_pct,
        already_rewarded: false,
        rewarded: true,
        amount: entry.earning_unit,
        ledger_entry_id: Some(entry.id),
    })
}

fn parse_bucket_type(s: &str) -> Result<BucketType, AppError> {
    match s {
        "earned_credit" | "EarnedCredit" => Ok(BucketType::EarnedCredit),
        "cod_pending" | "CodPending" => Ok(BucketType::CodPending),
        "gift_card" | "GiftCard" => Ok(BucketType::GiftCard),
        "customer_funded" | "CustomerFunded" => Ok(BucketType::CustomerFunded),
        "referral_reward" | "ReferralReward" => Ok(BucketType::ReferralReward),
        "goodwill_credit" | "GoodwillCredit" => Ok(BucketType::GoodwillCredit),
        "membership_benefit" | "MembershipBenefit" => Ok(BucketType::MembershipBenefit),
        "refund_credit" | "RefundCredit" => Ok(BucketType::RefundCredit),
        other => Err(AppError::BadRequest(format!(
            "unknown bucket type: {other}"
        ))),
    }
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn check_and_award_milestones(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<MilestoneCheckResult, AppError> {
    let wallet =
        wallets::storage::get_or_create_wallet(pool, merchant_id, customer_id).await?;

    let milestones = storage::get_active_milestones(pool, merchant_id).await?;

    let order_stats = storage::get_customer_order_stats(pool, merchant_id, customer_id).await?;

    let (total_orders, total_spend) = match order_stats {
        Some(stats) => (stats.total_orders, stats.total_spend),
        None => {
            return Ok(MilestoneCheckResult {
                customer_id,
                milestones_achieved: Vec::new(),
            });
        }
    };

    let mut achieved = Vec::new();

    for milestone in &milestones {
        let already = storage::has_achieved_milestone(pool, customer_id, milestone.id).await?;
        if already {
            continue;
        }

        let threshold_crossed = match milestone.milestone_type.as_str() {
            "order_count" => total_orders as f64 >= milestone.threshold,
            "lifetime_spend" => total_spend >= milestone.threshold,
            _ => false,
        };

        if !threshold_crossed {
            continue;
        }

        let hash_input = format!("{}{}{}", merchant_id, customer_id, milestone.id);
        let mut hasher = Sha256::new();
        hasher.update(hash_input.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        let idempotency_key = format!("milestone:{hash}");

        let new_entry = NewLedgerEntry {
            wallet_id: wallet.id,
            bucket_type: BucketType::EarnedCredit,
            movement_type: MovementType::In,
            earning_unit: milestone.reward_amount,
            currency_equivalent: milestone.reward_amount,
            conversion_rate: 1.0,
            event_id: None,
            rule_snapshot_id: None,
            campaign_snapshot_id: None,
            actor_type: ActorType::System,
            actor_id: Some("milestone_reward".to_string()),
            payment_reference: Some(format!("milestone:{}", milestone.name)),
            transfer_id: None,
            constraints: serde_json::json!({}),
            expires_at: None,
        };

        let entry = ledger::storage::create_entry(pool, new_entry, idempotency_key).await?;

        storage::record_milestone_achievement(
            pool,
            merchant_id,
            customer_id,
            milestone.id,
            entry.id,
        )
        .await?;

        achieved.push(MilestoneAchievementEntry {
            milestone_name: milestone.name.clone(),
            reward_amount: milestone.reward_amount,
            ledger_entry_id: entry.id,
        });
    }

    Ok(MilestoneCheckResult {
        customer_id,
        milestones_achieved: achieved,
    })
}
