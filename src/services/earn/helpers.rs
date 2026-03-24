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
use crate::services::loyalty;
use crate::services::rules;
use crate::services::rules::types::EvaluationContext;
use crate::services::cod;
use crate::services::wallets;

use super::storage;
use super::types::{
    ActiveStreak, AssignMembershipRequest, AssignMembershipResult, BirthdayBonusEntry,
    BirthdayBonusResult, CreateWheelRequest, EarnEntry, EarnResult, ManualCreditRequest,
    ManualCreditResult, MembershipStatus, MilestoneAchievementEntry, MilestoneCheckResult,
    NewsletterSignupRequest, NewsletterSignupResult, ProfileCompletionRequest,
    ProfileCompletionResult, SpinRequest, SpinResult, StreakAchievementEntry, StreakCheckResult,
    WheelWithSegments,
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

    let mut eval_results = rules::helpers::evaluate_rules(pool, &context).await?;

    let loyalty_mult = loyalty::helpers::get_earn_multiplier(pool, event.merchant_id, customer.id).await.unwrap_or(1.0);
    let membership_status = get_membership_status(pool, event.merchant_id, customer.id).await.unwrap_or_default();
    let membership_mult = if membership_status.is_active { membership_status.earn_rate_multiplier } else { 1.0 };
    let effective_mult = loyalty_mult.max(membership_mult);

    if effective_mult > 1.0 {
        for eval in &mut eval_results {
            if eval.matched && eval.earning_unit > 0.0 {
                eval.earning_unit *= effective_mult;
                eval.currency_equivalent *= effective_mult;
            }
        }
    }

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

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn check_and_award_streaks(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<StreakCheckResult, AppError> {
    let wallet =
        wallets::storage::get_or_create_wallet(pool, merchant_id, customer_id).await?;

    let configs = storage::get_active_streak_configs(pool, merchant_id).await?;

    let mut streaks_achieved = Vec::new();
    let mut active_streaks = Vec::new();

    let now = Utc::now();

    for config in &configs {
        let orders_in_window =
            storage::count_recent_orders(pool, merchant_id, customer_id, config.window_days)
                .await?;

        let progress_pct = if config.required_orders > 0 {
            ((orders_in_window as f64) / (config.required_orders as f64) * 100.0).min(100.0)
        } else {
            0.0
        };

        active_streaks.push(ActiveStreak {
            streak_name: config.name.clone(),
            required_orders: config.required_orders,
            orders_in_window,
            window_days: config.window_days,
            progress_pct,
        });

        if orders_in_window >= config.required_orders as i64 {
            let window_start = now
                .checked_sub_days(Days::new(config.window_days as u64))
                .unwrap_or(now);
            let window_end = now;

            let already = storage::has_streak_achievement_in_window(
                pool,
                customer_id,
                config.id,
                window_start,
            )
            .await?;

            if already {
                continue;
            }

            let window_start_date = window_start.format("%Y-%m-%d").to_string();
            let hash_input = format!(
                "{}{}{}{}",
                merchant_id, customer_id, config.id, window_start_date
            );
            let mut hasher = Sha256::new();
            hasher.update(hash_input.as_bytes());
            let hash = format!("{:x}", hasher.finalize());
            let idempotency_key = format!("streak:{hash}");

            let new_entry = NewLedgerEntry {
                wallet_id: wallet.id,
                bucket_type: BucketType::EarnedCredit,
                movement_type: MovementType::In,
                earning_unit: config.reward_amount,
                currency_equivalent: config.reward_amount,
                conversion_rate: 1.0,
                event_id: None,
                rule_snapshot_id: None,
                campaign_snapshot_id: None,
                actor_type: ActorType::System,
                actor_id: Some("streak_reward".to_string()),
                payment_reference: Some(format!("streak:{}", config.name)),
                transfer_id: None,
                constraints: serde_json::json!({}),
                expires_at: None,
            };

            let entry =
                ledger::storage::create_entry(pool, new_entry, idempotency_key).await?;

            storage::record_streak_achievement(
                pool,
                merchant_id,
                customer_id,
                config.id,
                entry.id,
                window_start,
                window_end,
            )
            .await?;

            streaks_achieved.push(StreakAchievementEntry {
                streak_name: config.name.clone(),
                reward_amount: config.reward_amount,
                ledger_entry_id: entry.id,
            });
        }
    }

    Ok(StreakCheckResult {
        customer_id,
        streaks_achieved,
        active_streaks,
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

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_wheel(
    pool: &PgPool,
    req: CreateWheelRequest,
) -> Result<WheelWithSegments, AppError> {
    let name = req.name.as_deref().unwrap_or("Lucky Wheel");
    let daily_spin_limit = req.daily_spin_limit.unwrap_or(1);

    let config =
        storage::create_wheel_config(pool, req.merchant_id, name, daily_spin_limit).await?;

    let mut segments = Vec::with_capacity(req.segments.len());
    for (i, seg) in req.segments.iter().enumerate() {
        let color = seg.color.as_deref().unwrap_or("#7c6aff");
        let segment = storage::create_wheel_segment(
            pool,
            config.id,
            &seg.label,
            seg.reward_amount,
            seg.probability,
            color,
            i as i32,
        )
        .await?;
        segments.push(segment);
    }

    Ok(WheelWithSegments { config, segments })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn spin_wheel(pool: &PgPool, req: SpinRequest) -> Result<SpinResult, AppError> {
    let config = storage::get_wheel_config(pool, req.merchant_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound("no spin wheel configured for this merchant".to_string())
        })?;

    if !config.is_active {
        return Err(AppError::BadRequest(
            "spin wheel is not active".to_string(),
        ));
    }

    let spins_today = storage::count_spins_today(pool, req.merchant_id, req.customer_id).await?;
    if spins_today >= config.daily_spin_limit as i64 {
        return Err(AppError::BadRequest(
            "no spins remaining today".to_string(),
        ));
    }

    let segments = storage::get_wheel_segments(pool, config.id).await?;
    if segments.is_empty() {
        return Err(AppError::BadRequest(
            "wheel has no segments configured".to_string(),
        ));
    }

    let total_weight: f64 = segments.iter().map(|s| s.probability).sum();
    if total_weight <= 0.0 {
        return Err(AppError::Internal(
            "total segment weight is zero".to_string(),
        ));
    }

    let random_value =
        (Uuid::new_v4().as_u128() % 10000) as f64 / 10000.0 * total_weight;

    let mut accumulated = 0.0_f64;
    let mut winning_segment = &segments[0];
    for segment in &segments {
        accumulated += segment.probability;
        if random_value < accumulated {
            winning_segment = segment;
            break;
        }
    }

    let mut ledger_entry_id: Option<Uuid> = None;

    if winning_segment.reward_amount > 0.0 {
        let wallet =
            wallets::storage::get_or_create_wallet(pool, req.merchant_id, req.customer_id).await?;

        let idempotency_key = format!("spin:{}:{}", req.customer_id, Uuid::new_v4());

        let new_entry = NewLedgerEntry {
            wallet_id: wallet.id,
            bucket_type: BucketType::EarnedCredit,
            movement_type: MovementType::In,
            earning_unit: winning_segment.reward_amount,
            currency_equivalent: winning_segment.reward_amount,
            conversion_rate: 1.0,
            event_id: None,
            rule_snapshot_id: None,
            campaign_snapshot_id: None,
            actor_type: ActorType::System,
            actor_id: Some("spin_wheel".to_string()),
            payment_reference: Some(format!("spin:{}", winning_segment.label)),
            transfer_id: None,
            constraints: serde_json::json!({}),
            expires_at: None,
        };

        let entry = ledger::storage::create_entry(pool, new_entry, idempotency_key).await?;
        ledger_entry_id = Some(entry.id);
    }

    storage::record_spin_result(
        pool,
        req.merchant_id,
        req.customer_id,
        winning_segment.id,
        winning_segment.reward_amount,
        ledger_entry_id,
    )
    .await?;

    let spins_remaining_today =
        config.daily_spin_limit - (spins_today as i32 + 1);

    Ok(SpinResult {
        segment: winning_segment.clone(),
        reward_amount: winning_segment.reward_amount,
        ledger_entry_id,
        spins_remaining_today,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn assign_membership(
    pool: &PgPool,
    req: AssignMembershipRequest,
) -> Result<AssignMembershipResult, AppError> {
    let tier = loyalty::storage::get_tier_by_id(pool, req.tier_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("loyalty tier {} not found", req.tier_id)))?;

    let customer_tier = loyalty::storage::get_customer_tier(pool, req.customer_id, req.merchant_id).await?;
    if let Some(ref ct) = customer_tier {
        let earned_tier = loyalty::storage::get_tier_by_id(pool, ct.tier_id).await?;
        if let Some(ref et) = earned_tier {
            if et.rank >= tier.rank {
                return Err(AppError::BadRequest(format!(
                    "customer already has {} (rank {}), cannot assign {} (rank {})",
                    et.name, et.rank, tier.name, tier.rank
                )));
            }
        }
    }

    let existing = storage::get_customer_membership(pool, req.merchant_id, req.customer_id).await?;
    if let Some(ref membership) = existing {
        if membership.status == "active" && membership.tier_id == req.tier_id {
            return Ok(AssignMembershipResult {
                membership: membership.clone(),
                tier_name: tier.name.clone(),
                earn_rate_multiplier: tier.earn_rate_multiplier,
                is_new: false,
                message: "already assigned to this tier".to_string(),
            });
        }
    }

    let now = Utc::now();
    let expires_at = now.checked_add_days(Days::new(365)).unwrap_or(now);

    let membership =
        storage::subscribe_customer(pool, req.merchant_id, req.customer_id, req.tier_id, expires_at)
            .await?;

    Ok(AssignMembershipResult {
        membership,
        tier_name: tier.name,
        earn_rate_multiplier: tier.earn_rate_multiplier,
        is_new: true,
        message: "membership assigned successfully".to_string(),
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_membership_status(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<MembershipStatus, AppError> {
    let membership = storage::get_customer_membership(pool, merchant_id, customer_id).await?;

    let Some(membership) = membership else {
        return Ok(MembershipStatus::default());
    };

    if membership.status == "active" && membership.expires_at < Utc::now() {
        storage::expire_membership(pool, membership.id).await?;
        let tier = loyalty::storage::get_tier_by_id(pool, membership.tier_id).await?;
        let expired = super::types::CustomerMembership {
            status: "expired".to_string(),
            ..membership
        };
        return Ok(MembershipStatus {
            membership: Some(expired),
            tier_name: tier.map(|t| t.name),
            earn_rate_multiplier: 1.0,
            is_active: false,
            days_remaining: 0,
        });
    }

    let tier = loyalty::storage::get_tier_by_id(pool, membership.tier_id).await?;
    let is_active = membership.status == "active";
    let days_remaining = if is_active {
        (membership.expires_at - Utc::now()).num_days().max(0)
    } else {
        0
    };
    let multiplier = if is_active {
        tier.as_ref().map(|t| t.earn_rate_multiplier).unwrap_or(1.0)
    } else {
        1.0
    };

    Ok(MembershipStatus {
        membership: Some(membership),
        tier_name: tier.map(|t| t.name),
        earn_rate_multiplier: multiplier,
        is_active,
        days_remaining,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn cancel_membership_by_id(
    pool: &PgPool,
    membership_id: Uuid,
) -> Result<super::types::CustomerMembership, AppError> {
    storage::cancel_membership(pool, membership_id).await
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn upgrade_membership(
    pool: &PgPool,
    membership_id: Uuid,
    tier_id: Uuid,
) -> Result<AssignMembershipResult, AppError> {
    let _membership = storage::get_customer_membership_by_id(pool, membership_id).await?;

    let tier = loyalty::storage::get_tier_by_id(pool, tier_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("loyalty tier {tier_id} not found")))?;

    let updated = storage::upgrade_membership_tier(pool, membership_id, tier_id).await?;

    Ok(AssignMembershipResult {
        membership: updated,
        tier_name: tier.name,
        earn_rate_multiplier: tier.earn_rate_multiplier,
        is_new: false,
        message: "membership tier upgraded successfully".to_string(),
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn extend_membership(
    pool: &PgPool,
    membership_id: Uuid,
    days: i64,
) -> Result<super::types::CustomerMembership, AppError> {
    let membership = storage::get_customer_membership_by_id(pool, membership_id).await?;

    let new_expires_at = membership.expires_at + chrono::Duration::days(days);

    storage::renew_membership(pool, membership_id, new_expires_at).await
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn renew_membership(
    pool: &PgPool,
    membership_id: Uuid,
) -> Result<super::types::CustomerMembership, AppError> {
    let _membership = storage::get_customer_membership_by_id(pool, membership_id).await?;

    let now = Utc::now();
    let new_expires_at = now.checked_add_days(Days::new(365)).unwrap_or(now);

    storage::renew_membership(pool, membership_id, new_expires_at).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::events::types::{ShopifyCustomer, ShopifyOrderPayload};
    use uuid::Uuid;

    fn make_payload(customer_phone: Option<&str>, order_phone: Option<&str>) -> ShopifyOrderPayload {
        ShopifyOrderPayload {
            id: 1,
            order_number: 1001,
            email: None,
            phone: order_phone.map(|s| s.to_string()),
            total_price: "1000.00".to_string(),
            currency: "INR".to_string(),
            financial_status: "paid".to_string(),
            gateway: None,
            payment_gateway_names: None,
            customer: Some(ShopifyCustomer {
                id: 1,
                email: None,
                phone: customer_phone.map(|s| s.to_string()),
                first_name: None,
                last_name: None,
            }),
            line_items: None,
        }
    }

    // -----------------------------------------------------------------------
    // parse_bucket_type
    // -----------------------------------------------------------------------

    #[test]
    fn parse_bucket_type_snake_case() {
        assert_eq!(parse_bucket_type("earned_credit").unwrap(), BucketType::EarnedCredit);
        assert_eq!(parse_bucket_type("cod_pending").unwrap(), BucketType::CodPending);
        assert_eq!(parse_bucket_type("gift_card").unwrap(), BucketType::GiftCard);
        assert_eq!(parse_bucket_type("customer_funded").unwrap(), BucketType::CustomerFunded);
        assert_eq!(parse_bucket_type("referral_reward").unwrap(), BucketType::ReferralReward);
        assert_eq!(parse_bucket_type("goodwill_credit").unwrap(), BucketType::GoodwillCredit);
        assert_eq!(parse_bucket_type("membership_benefit").unwrap(), BucketType::MembershipBenefit);
        assert_eq!(parse_bucket_type("refund_credit").unwrap(), BucketType::RefundCredit);
    }

    #[test]
    fn parse_bucket_type_pascal_case() {
        assert_eq!(parse_bucket_type("EarnedCredit").unwrap(), BucketType::EarnedCredit);
        assert_eq!(parse_bucket_type("CodPending").unwrap(), BucketType::CodPending);
        assert_eq!(parse_bucket_type("GiftCard").unwrap(), BucketType::GiftCard);
        assert_eq!(parse_bucket_type("CustomerFunded").unwrap(), BucketType::CustomerFunded);
        assert_eq!(parse_bucket_type("ReferralReward").unwrap(), BucketType::ReferralReward);
        assert_eq!(parse_bucket_type("GoodwillCredit").unwrap(), BucketType::GoodwillCredit);
        assert_eq!(parse_bucket_type("MembershipBenefit").unwrap(), BucketType::MembershipBenefit);
        assert_eq!(parse_bucket_type("RefundCredit").unwrap(), BucketType::RefundCredit);
    }

    #[test]
    fn parse_bucket_type_unknown() {
        assert!(parse_bucket_type("unknown").is_err());
        assert!(parse_bucket_type("").is_err());
    }

    // -----------------------------------------------------------------------
    // extract_customer_phone
    // -----------------------------------------------------------------------

    #[test]
    fn phone_from_customer_object() {
        let payload = make_payload(Some("+919876543210"), Some("+911234567890"));
        assert_eq!(extract_customer_phone(&payload).unwrap(), "+919876543210");
    }

    #[test]
    fn phone_fallback_to_order() {
        let payload = make_payload(None, Some("+911234567890"));
        assert_eq!(extract_customer_phone(&payload).unwrap(), "+911234567890");
    }

    #[test]
    fn phone_empty_customer_falls_to_order() {
        let payload = make_payload(Some(""), Some("+911234567890"));
        assert_eq!(extract_customer_phone(&payload).unwrap(), "+911234567890");
    }

    #[test]
    fn phone_none_everywhere_errors() {
        let payload = make_payload(None, None);
        assert!(extract_customer_phone(&payload).is_err());
    }

    #[test]
    fn phone_empty_everywhere_errors() {
        let payload = make_payload(Some(""), Some(""));
        assert!(extract_customer_phone(&payload).is_err());
    }

    // -----------------------------------------------------------------------
    // extract_customer_email
    // -----------------------------------------------------------------------

    #[test]
    fn email_from_customer_object() {
        let mut payload = make_payload(None, None);
        payload.customer.as_mut().unwrap().email = Some("test@example.com".to_string());
        assert_eq!(extract_customer_email(&payload).unwrap(), "test@example.com");
    }

    #[test]
    fn email_fallback_to_order() {
        let mut payload = make_payload(None, None);
        payload.customer.as_mut().unwrap().email = None;
        payload.email = Some("order@example.com".to_string());
        assert_eq!(extract_customer_email(&payload).unwrap(), "order@example.com");
    }

    #[test]
    fn email_empty_returns_none() {
        let mut payload = make_payload(None, None);
        payload.customer.as_mut().unwrap().email = Some("".to_string());
        payload.email = Some("".to_string());
        assert!(extract_customer_email(&payload).is_none());
    }

    // -----------------------------------------------------------------------
    // extract_customer_name
    // -----------------------------------------------------------------------

    #[test]
    fn name_from_first_and_last() {
        let mut payload = make_payload(None, None);
        payload.customer.as_mut().unwrap().first_name = Some("John".to_string());
        payload.customer.as_mut().unwrap().last_name = Some("Doe".to_string());
        assert_eq!(extract_customer_name(&payload).unwrap(), "John Doe");
    }

    #[test]
    fn name_first_only() {
        let mut payload = make_payload(None, None);
        payload.customer.as_mut().unwrap().first_name = Some("John".to_string());
        assert_eq!(extract_customer_name(&payload).unwrap(), "John");
    }

    #[test]
    fn name_both_empty_returns_none() {
        let payload = make_payload(None, None);
        assert!(extract_customer_name(&payload).is_none());
    }

    // -----------------------------------------------------------------------
    // generate_earn_idempotency_key
    // -----------------------------------------------------------------------

    #[test]
    fn idempotency_key_with_rule_snapshot() {
        let event_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let rule_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
        let key = generate_earn_idempotency_key(event_id, Some(rule_id));
        assert_eq!(key, format!("earn:{event_id}:{rule_id}"));
    }

    #[test]
    fn idempotency_key_without_rule_snapshot() {
        let event_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let key = generate_earn_idempotency_key(event_id, None);
        assert_eq!(key, format!("earn:{event_id}"));
    }
}
