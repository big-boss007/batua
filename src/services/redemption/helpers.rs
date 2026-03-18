use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::ledger::storage as ledger_storage;
use crate::services::ledger::types::{ActorType, BucketType, MovementType, NewLedgerEntry};
use crate::services::wallets::storage as wallet_storage;

use super::storage;
use super::types::{
    BucketDebit, BucketEligibility, OrderContext, RedemptionEligibility, RedemptionResponse,
    RedemptionState, WalletPolicy,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn evaluate_eligibility(
    pool: &PgPool,
    wallet_id: Uuid,
    order_context: &OrderContext,
) -> Result<RedemptionEligibility, AppError> {
    let wallet = wallet_storage::get_wallet(pool, wallet_id).await?;
    let balance = ledger_storage::get_balance(pool, wallet_id).await?;
    let policies = storage::get_wallet_policies(pool, wallet.merchant_id).await?;

    let mut buckets = Vec::new();
    let mut total_eligible = 0.0;

    for bucket_balance in &balance.buckets {
        let policy = policies
            .iter()
            .find(|p| p.bucket_type == bucket_balance.bucket_type);

        let eligible = evaluate_bucket_eligibility(
            &bucket_balance.bucket_type,
            bucket_balance.spendable,
            policy,
            order_context,
        );

        if eligible > 0.0 {
            let constraints = build_constraint_summary(policy);
            buckets.push(BucketEligibility {
                bucket_type: bucket_balance.bucket_type.clone(),
                eligible_amount: eligible,
                constraints,
            });
            total_eligible += eligible;
        }
    }

    Ok(RedemptionEligibility {
        total_eligible,
        buckets,
    })
}

#[tracing::instrument]
fn evaluate_bucket_eligibility(
    bucket_type: &BucketType,
    spendable: f64,
    policy: Option<&WalletPolicy>,
    order_context: &OrderContext,
) -> f64 {
    if spendable <= 0.0 {
        return 0.0;
    }

    let Some(policy) = policy else {
        return spendable;
    };

    if !policy.is_active {
        return 0.0;
    }

    if let Some(ref pm) = order_context.payment_method {
        if policy
            .excluded_payment_methods
            .iter()
            .any(|excluded| excluded.eq_ignore_ascii_case(pm))
        {
            return 0.0;
        }
    }

    let mut cap = spendable;

    if let Some(pct) = policy.max_per_order_pct {
        let pct_cap = order_context.order_amount * (pct / 100.0);
        cap = cap.min(pct_cap);
    }

    if let Some(fixed) = policy.max_per_order_fixed {
        cap = cap.min(fixed);
    }

    cap.max(0.0)
}

#[tracing::instrument]
fn build_constraint_summary(policy: Option<&WalletPolicy>) -> serde_json::Value {
    let Some(policy) = policy else {
        return serde_json::json!({});
    };

    serde_json::json!({
        "min_redemption": policy.min_redemption,
        "step_size": policy.step_size,
        "max_per_order_pct": policy.max_per_order_pct,
        "max_per_order_fixed": policy.max_per_order_fixed,
        "stackable_with_discounts": policy.stackable_with_discounts,
        "excluded_payment_methods": policy.excluded_payment_methods,
        "excluded_collections": policy.excluded_collections,
    })
}

#[tracing::instrument(err(Debug))]
pub fn validate_constraints(
    eligibility: &RedemptionEligibility,
    requested_amount: f64,
    policies: &[WalletPolicy],
    discount_codes: &[String],
) -> Result<f64, AppError> {
    if requested_amount <= 0.0 {
        return Err(AppError::BadRequest(
            "requested amount must be positive".to_string(),
        ));
    }

    if requested_amount > eligibility.total_eligible {
        return Err(AppError::BadRequest(format!(
            "requested amount {requested_amount} exceeds total eligible {}",
            eligibility.total_eligible
        )));
    }

    if !discount_codes.is_empty() {
        let all_stackable = policies
            .iter()
            .all(|p| p.stackable_with_discounts);

        if !all_stackable {
            return Err(AppError::BadRequest(
                "wallet credits cannot be stacked with discount codes for this merchant"
                    .to_string(),
            ));
        }
    }

    let mut validated = requested_amount;

    for policy in policies {
        if let Some(min) = policy.min_redemption {
            if validated < min && validated > 0.0 {
                return Err(AppError::BadRequest(format!(
                    "amount {validated} is below minimum redemption {min} for bucket {:?}",
                    policy.bucket_type
                )));
            }
        }

        if let Some(step) = policy.step_size {
            if step > 0.0 {
                validated = (validated / step).floor() * step;
            }
        }
    }

    if validated <= 0.0 {
        return Err(AppError::BadRequest(
            "validated amount is zero after applying constraints".to_string(),
        ));
    }

    Ok(validated)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn execute_redemption(
    pool: &PgPool,
    redemption_id: Uuid,
) -> Result<RedemptionResponse, AppError> {
    let redemption = storage::get_redemption(pool, redemption_id).await?;

    if redemption.state != RedemptionState::Initiated {
        return Err(AppError::Conflict(format!(
            "redemption {redemption_id} is in state {:?}, expected Initiated",
            redemption.state
        )));
    }

    storage::update_redemption_state(
        pool,
        redemption_id,
        &RedemptionState::Validating,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await?;

    let wallet = wallet_storage::get_wallet(pool, redemption.wallet_id).await?;
    let order_context = OrderContext {
        order_id: redemption.order_id.clone(),
        order_amount: redemption.order_amount,
        payment_method: redemption.payment_method.clone(),
        discount_codes: Vec::new(),
    };

    let eligibility =
        evaluate_eligibility(pool, redemption.wallet_id, &order_context).await?;

    let policies =
        storage::get_wallet_policies(pool, wallet.merchant_id).await?;

    let validated = match validate_constraints(
        &eligibility,
        redemption.requested_amount,
        &policies,
        &order_context.discount_codes,
    ) {
        Ok(amount) => amount,
        Err(e) => {
            storage::update_redemption_state(
                pool,
                redemption_id,
                &RedemptionState::Rejected,
                Some(eligibility.total_eligible),
                None,
                None,
                None,
                None,
                Some(&e.to_string()),
            )
            .await?;
            return Err(e);
        }
    };

    storage::update_redemption_state(
        pool,
        redemption_id,
        &RedemptionState::Committed,
        Some(eligibility.total_eligible),
        Some(validated),
        None,
        None,
        None,
        None,
    )
    .await?;

    let buckets_debited = create_debit_entries(
        pool,
        &redemption,
        &eligibility,
        validated,
        redemption_id,
    )
    .await?;

    let first_debit_id = buckets_debited.first().map(|b| b.entry_id);
    storage::update_redemption_state(
        pool,
        redemption_id,
        &RedemptionState::Applied,
        None,
        None,
        first_debit_id,
        None,
        None,
        None,
    )
    .await?;

    match apply_shopify_discount(pool, redemption_id, validated).await {
        Ok(discount_id) => {
            storage::update_redemption_state(
                pool,
                redemption_id,
                &RedemptionState::Completed,
                None,
                None,
                None,
                None,
                Some(&discount_id),
                None,
            )
            .await?;

            Ok(RedemptionResponse {
                redemption_id,
                state: RedemptionState::Completed,
                applied_amount: Some(validated),
                buckets_debited,
            })
        }
        Err(shopify_err) => {
            tracing::error!(
                redemption_id = %redemption_id,
                error = %shopify_err,
                "shopify discount application failed, initiating compensation"
            );

            storage::update_redemption_state(
                pool,
                redemption_id,
                &RedemptionState::Failed,
                None,
                None,
                None,
                None,
                None,
                Some(&format!("shopify failure: {shopify_err}")),
            )
            .await?;

            if let Err(comp_err) = compensate(pool, redemption_id).await {
                tracing::error!(
                    redemption_id = %redemption_id,
                    error = %comp_err,
                    "compensation also failed"
                );
            }

            Err(AppError::Internal(format!(
                "shopify discount failed for redemption {redemption_id}: {shopify_err}"
            )))
        }
    }
}

#[tracing::instrument(skip(pool), err(Debug))]
async fn create_debit_entries(
    pool: &PgPool,
    redemption: &super::types::RedemptionRequest,
    eligibility: &RedemptionEligibility,
    total_amount: f64,
    redemption_id: Uuid,
) -> Result<Vec<BucketDebit>, AppError> {
    let mut debits = Vec::new();
    let mut remaining = total_amount;

    for bucket in &eligibility.buckets {
        if remaining <= 0.0 {
            break;
        }

        let debit_amount = remaining.min(bucket.eligible_amount);
        remaining -= debit_amount;

        let idempotency_key = format!("redemption-{}-{:?}", redemption_id, bucket.bucket_type);

        let entry = ledger_storage::create_entry(
            pool,
            NewLedgerEntry {
                wallet_id: redemption.wallet_id,
                bucket_type: bucket.bucket_type.clone(),
                movement_type: MovementType::Out,
                earning_unit: debit_amount,
                currency_equivalent: debit_amount,
                conversion_rate: 1.0,
                event_id: None,
                rule_snapshot_id: None,
                campaign_snapshot_id: None,
                actor_type: ActorType::System,
                actor_id: Some(format!("redemption:{redemption_id}")),
                payment_reference: Some(redemption.order_id.clone()),
                transfer_id: None,
                constraints: serde_json::json!({
                    "redemption_id": redemption_id,
                    "order_id": redemption.order_id,
                }),
                expires_at: None,
            },
            idempotency_key,
        )
        .await?;

        debits.push(BucketDebit {
            bucket_type: bucket.bucket_type.clone(),
            amount: debit_amount,
            entry_id: entry.id,
        });
    }

    Ok(debits)
}

#[tracing::instrument(skip(pool), err(Debug))]
async fn apply_shopify_discount(
    pool: &PgPool,
    redemption_id: Uuid,
    amount: f64,
) -> Result<String, AppError> {
    let _ = (pool, amount);
    Ok(format!("stub-discount-{redemption_id}"))
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn compensate(pool: &PgPool, redemption_id: Uuid) -> Result<(), AppError> {
    let redemption = storage::get_redemption(pool, redemption_id).await?;

    match redemption.state {
        RedemptionState::Failed | RedemptionState::Applied | RedemptionState::Committed => {}
        _ => {
            return Err(AppError::BadRequest(format!(
                "cannot compensate redemption in state {:?}",
                redemption.state
            )));
        }
    }

    let wallet = wallet_storage::get_wallet(pool, redemption.wallet_id).await?;
    let policies =
        storage::get_wallet_policies(pool, wallet.merchant_id).await?;

    let order_context = OrderContext {
        order_id: redemption.order_id.clone(),
        order_amount: redemption.order_amount,
        payment_method: redemption.payment_method.clone(),
        discount_codes: Vec::new(),
    };

    let eligibility =
        evaluate_eligibility(pool, redemption.wallet_id, &order_context).await?;

    let applied = redemption.applied_amount.unwrap_or(0.0);
    if applied <= 0.0 {
        storage::update_redemption_state(
            pool,
            redemption_id,
            &RedemptionState::Compensated,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
        return Ok(());
    }

    let mut remaining = applied;
    let mut first_comp_id: Option<Uuid> = None;

    let bucket_types: Vec<_> = policies.iter().map(|p| p.bucket_type.clone()).collect();
    let target_buckets: Vec<_> = if bucket_types.is_empty() {
        eligibility
            .buckets
            .iter()
            .map(|b| b.bucket_type.clone())
            .collect()
    } else {
        bucket_types
    };

    for bucket_type in &target_buckets {
        if remaining <= 0.0 {
            break;
        }

        let credit_amount = remaining;
        remaining = 0.0;

        let idempotency_key =
            format!("compensation-{}-{:?}", redemption_id, bucket_type);

        let entry = ledger_storage::create_entry(
            pool,
            NewLedgerEntry {
                wallet_id: redemption.wallet_id,
                bucket_type: bucket_type.clone(),
                movement_type: MovementType::In,
                earning_unit: credit_amount,
                currency_equivalent: credit_amount,
                conversion_rate: 1.0,
                event_id: None,
                rule_snapshot_id: None,
                campaign_snapshot_id: None,
                actor_type: ActorType::System,
                actor_id: Some(format!("compensation:{redemption_id}")),
                payment_reference: Some(redemption.order_id.clone()),
                transfer_id: None,
                constraints: serde_json::json!({
                    "redemption_id": redemption_id,
                    "compensation": true,
                    "requires_review": true,
                }),
                expires_at: None,
            },
            idempotency_key,
        )
        .await?;

        if first_comp_id.is_none() {
            first_comp_id = Some(entry.id);
        }
    }

    storage::update_redemption_state(
        pool,
        redemption_id,
        &RedemptionState::Compensated,
        None,
        None,
        None,
        first_comp_id,
        None,
        None,
    )
    .await?;

    Ok(())
}
