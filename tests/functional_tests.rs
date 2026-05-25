use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use batua::error::AppError;
use batua::services::earn::helpers as earn_helpers;
use batua::services::earn::storage as earn_storage;
use batua::services::earn::types::{
    AssignMembershipRequest, CreateMilestoneRequest, CreateStreakConfigRequest,
    CreateWheelRequest, CreateSegmentRequest, ManualCreditRequest, SpinRequest,
};
use batua::services::events::storage as event_storage;
use batua::services::events::types::IngestEventRequest;
use batua::services::gift_cards::helpers as gc_helpers;
use batua::services::gift_cards::types::{
    BulkCardItem, BulkIssueRequest, ClaimGiftCardRequest, IssueGiftCardRequest,
    RedeemGiftCardRequest,
};
use batua::services::identity::storage as identity_storage;
use batua::services::identity::types::ResolveIdentityRequest;
use batua::services::ledger::storage as ledger_storage;
use batua::services::ledger::types::{
    ActorType, BucketType, MovementType, NewLedgerEntry,
};
use batua::services::loyalty::helpers as loyalty_helpers;
use batua::services::loyalty::storage as loyalty_storage;
use batua::services::loyalty::types::{CreateProgramRequest, CreateTierRequest};
use batua::services::redemption::helpers as redemption_helpers;
use batua::services::redemption::storage as redemption_storage;
use batua::services::redemption::types::{OrderContext, RedemptionState};
use batua::services::rules::storage as rules_storage;
use batua::services::rules::types::CreateRuleRequest;
use batua::services::wallets::storage as wallet_storage;

// ---------------------------------------------------------------------------
// Test infrastructure
// ---------------------------------------------------------------------------

async fn get_test_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .unwrap_or_else(|_| "postgres://chirag@localhost/batua_test".to_string());
    PgPool::connect(&url)
        .await
        .expect("Failed to connect to test database")
}

async fn create_test_merchant(pool: &PgPool) -> Uuid {
    let id: (Uuid,) = sqlx::query_as(
        "INSERT INTO merchants (external_id, name, domain, currency) \
         VALUES ($1, $2, $3, 'INR') RETURNING id",
    )
    .bind(format!("func-{}", Uuid::new_v4()))
    .bind("Functional Test Merchant")
    .bind("func-test.myshopify.com")
    .fetch_one(pool)
    .await
    .expect("create merchant");
    id.0
}

async fn create_test_customer(pool: &PgPool) -> Uuid {
    let unique = Uuid::new_v4().as_u128() % 9_000_000_000 + 1_000_000_000;
    let phone = format!("{unique}");
    let (customer, _) = identity_storage::resolve_or_create(
        pool,
        &ResolveIdentityRequest {
            phone,
            email: None,
            name: Some("Functional Test User".to_string()),
            external_id: None,
        },
    )
    .await
    .expect("create customer");
    customer.id
}

async fn create_test_wallet(pool: &PgPool, merchant_id: Uuid, customer_id: Uuid) -> Uuid {
    let wallet = wallet_storage::get_or_create_wallet(pool, merchant_id, customer_id)
        .await
        .expect("create wallet");
    wallet.id
}

fn unique_key() -> String {
    format!("func-{}", Uuid::new_v4())
}

fn make_in_entry(wallet_id: Uuid, bucket: BucketType, amount: f64) -> NewLedgerEntry {
    NewLedgerEntry {
        wallet_id,
        bucket_type: bucket,
        movement_type: MovementType::In,
        earning_unit: amount,
        currency_equivalent: amount,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: None,
        payment_reference: None,
        transfer_id: None,
        constraints: json!({}),
        expires_at: None,
    }
}

// ===========================================================================
// Gift Card Lifecycle Tests
// ===========================================================================

#[tokio::test]
async fn gc_issue_claim_redeem_full() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let issued = gc_helpers::issue_gift_card(
        &pool,
        IssueGiftCardRequest {
            merchant_id: merchant,
            amount: 500.0,
            actor_type: "system".to_string(),
            actor_id: Some("test".to_string()),
            payment_reference: None,
            expires_at: None,
        },
    )
    .await
    .expect("issue");

    assert!(issued.code.starts_with("BRZE-"));
    assert!((issued.initial_amount - 500.0).abs() < f64::EPSILON);

    let claimed = gc_helpers::claim_gift_card(
        &pool,
        ClaimGiftCardRequest {
            code: issued.code.clone(),
            customer_id: customer,
        },
    )
    .await
    .expect("claim");

    assert!(claimed.is_claimed);

    let redeemed = gc_helpers::redeem_gift_card(
        &pool,
        RedeemGiftCardRequest {
            code: issued.code.clone(),
            amount: 500.0,
            order_id: "order-gc-001".to_string(),
        },
    )
    .await
    .expect("redeem full");

    assert!((redeemed.current_amount - 0.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn gc_partial_redeem_has_remaining() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let issued = gc_helpers::issue_gift_card(
        &pool,
        IssueGiftCardRequest {
            merchant_id: merchant,
            amount: 500.0,
            actor_type: "system".to_string(),
            actor_id: None,
            payment_reference: None,
            expires_at: None,
        },
    )
    .await
    .expect("issue");

    gc_helpers::claim_gift_card(
        &pool,
        ClaimGiftCardRequest {
            code: issued.code.clone(),
            customer_id: customer,
        },
    )
    .await
    .expect("claim");

    let redeemed = gc_helpers::redeem_gift_card(
        &pool,
        RedeemGiftCardRequest {
            code: issued.code.clone(),
            amount: 200.0,
            order_id: "order-gc-002".to_string(),
        },
    )
    .await
    .expect("partial redeem");

    assert!(
        (redeemed.current_amount - 300.0).abs() < f64::EPSILON,
        "remaining should be 300, got {}",
        redeemed.current_amount
    );
}

#[tokio::test]
async fn gc_redeem_exceeds_balance_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let issued = gc_helpers::issue_gift_card(
        &pool,
        IssueGiftCardRequest {
            merchant_id: merchant,
            amount: 100.0,
            actor_type: "system".to_string(),
            actor_id: None,
            payment_reference: None,
            expires_at: None,
        },
    )
    .await
    .expect("issue");

    gc_helpers::claim_gift_card(
        &pool,
        ClaimGiftCardRequest {
            code: issued.code.clone(),
            customer_id: customer,
        },
    )
    .await
    .expect("claim");

    let result = gc_helpers::redeem_gift_card(
        &pool,
        RedeemGiftCardRequest {
            code: issued.code,
            amount: 200.0,
            order_id: "order-gc-003".to_string(),
        },
    )
    .await;

    assert!(result.is_err(), "redeeming more than balance must fail");
}

#[tokio::test]
async fn gc_double_claim_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let issued = gc_helpers::issue_gift_card(
        &pool,
        IssueGiftCardRequest {
            merchant_id: merchant,
            amount: 100.0,
            actor_type: "system".to_string(),
            actor_id: None,
            payment_reference: None,
            expires_at: None,
        },
    )
    .await
    .expect("issue");

    gc_helpers::claim_gift_card(
        &pool,
        ClaimGiftCardRequest {
            code: issued.code.clone(),
            customer_id: customer,
        },
    )
    .await
    .expect("first claim");

    let second = gc_helpers::claim_gift_card(
        &pool,
        ClaimGiftCardRequest {
            code: issued.code,
            customer_id: customer,
        },
    )
    .await;

    assert!(second.is_err(), "double claim must be rejected");
}

#[tokio::test]
async fn gc_issue_zero_amount_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    let result = gc_helpers::issue_gift_card(
        &pool,
        IssueGiftCardRequest {
            merchant_id: merchant,
            amount: 0.0,
            actor_type: "system".to_string(),
            actor_id: None,
            payment_reference: None,
            expires_at: None,
        },
    )
    .await;

    assert!(result.is_err(), "zero amount issue must be rejected");
}

#[tokio::test]
async fn gc_issue_negative_amount_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    let result = gc_helpers::issue_gift_card(
        &pool,
        IssueGiftCardRequest {
            merchant_id: merchant,
            amount: -100.0,
            actor_type: "system".to_string(),
            actor_id: None,
            payment_reference: None,
            expires_at: None,
        },
    )
    .await;

    assert!(result.is_err(), "negative amount issue must be rejected");
}

#[tokio::test]
async fn gc_claim_expired_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let past = (Utc::now() - Duration::hours(1)).to_rfc3339();
    let issued = gc_helpers::issue_gift_card(
        &pool,
        IssueGiftCardRequest {
            merchant_id: merchant,
            amount: 100.0,
            actor_type: "system".to_string(),
            actor_id: None,
            payment_reference: None,
            expires_at: Some(past),
        },
    )
    .await
    .expect("issue with past expiry");

    let result = gc_helpers::claim_gift_card(
        &pool,
        ClaimGiftCardRequest {
            code: issued.code,
            customer_id: customer,
        },
    )
    .await;

    assert!(result.is_err(), "claiming expired gift card must fail");
}

#[tokio::test]
async fn gc_redeem_zero_amount_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    let issued = gc_helpers::issue_gift_card(
        &pool,
        IssueGiftCardRequest {
            merchant_id: merchant,
            amount: 100.0,
            actor_type: "system".to_string(),
            actor_id: None,
            payment_reference: None,
            expires_at: None,
        },
    )
    .await
    .expect("issue");

    let result = gc_helpers::redeem_gift_card(
        &pool,
        RedeemGiftCardRequest {
            code: issued.code,
            amount: 0.0,
            order_id: "order-gc-zero".to_string(),
        },
    )
    .await;

    assert!(result.is_err(), "zero amount redeem must be rejected");
}

#[tokio::test]
async fn gc_bulk_issue_all_unique_codes() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let batch_id = Uuid::new_v4();

    let result = gc_helpers::bulk_issue(
        &pool,
        BulkIssueRequest {
            merchant_id: merchant,
            batch_id,
            cards: (0..5)
                .map(|_| BulkCardItem { amount: 100.0, recipient_phone: None, recipient_email: None })
                .collect(),
        },
    )
    .await
    .expect("bulk issue");

    assert_eq!(result.total_issued, 5);
    assert_eq!(result.total_skipped, 0);

    let codes: std::collections::HashSet<_> = result.cards.iter().map(|c| c.code.clone()).collect();
    assert_eq!(codes.len(), 5, "all 5 codes must be unique");
}

#[tokio::test]
async fn gc_bulk_issue_idempotent_replay() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let batch_id = Uuid::new_v4();

    let first = gc_helpers::bulk_issue(
        &pool,
        BulkIssueRequest {
            merchant_id: merchant,
            batch_id,
            cards: vec![BulkCardItem { amount: 100.0, recipient_phone: None, recipient_email: None }],
        },
    )
    .await
    .expect("first issue");

    let second = gc_helpers::bulk_issue(
        &pool,
        BulkIssueRequest {
            merchant_id: merchant,
            batch_id,
            cards: vec![BulkCardItem { amount: 100.0, recipient_phone: None, recipient_email: None }],
        },
    )
    .await
    .expect("replay");

    assert_eq!(first.cards[0].code, second.cards[0].code);
    assert_eq!(second.total_skipped, 1);
    assert_eq!(second.total_issued, 0);
}

#[tokio::test]
async fn gc_redeem_without_claim_uses_bearer_wallet() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    let issued = gc_helpers::issue_gift_card(
        &pool,
        IssueGiftCardRequest {
            merchant_id: merchant,
            amount: 200.0,
            actor_type: "system".to_string(),
            actor_id: None,
            payment_reference: None,
            expires_at: None,
        },
    )
    .await
    .expect("issue");

    let redeemed = gc_helpers::redeem_gift_card(
        &pool,
        RedeemGiftCardRequest {
            code: issued.code,
            amount: 100.0,
            order_id: "order-gc-bearer".to_string(),
        },
    )
    .await
    .expect("redeem without claim");

    assert!(
        (redeemed.current_amount - 100.0).abs() < f64::EPSILON,
        "remaining should be 100"
    );
}

// ===========================================================================
// Loyalty Tier Evaluation Tests
// ===========================================================================

async fn create_program_with_tiers(
    pool: &PgPool,
    merchant_id: Uuid,
    criteria: &str,
    period_days: Option<i32>,
    tiers: Vec<(&str, i32, f64, f64)>,
) -> Uuid {
    let program = loyalty_storage::create_program(
        pool,
        &CreateProgramRequest {
            merchant_id,
            name: "Test Program".to_string(),
            evaluation_criteria: criteria.to_string(),
            evaluation_period_days: period_days,
        },
    )
    .await
    .expect("create program");

    for (name, rank, threshold, multiplier) in tiers {
        loyalty_storage::create_tier(
            pool,
            &CreateTierRequest {
                program_id: program.id,
                name: name.to_string(),
                rank,
                threshold,
                earn_rate_multiplier: multiplier,
                benefits: json!({}),
            },
        )
        .await
        .expect("create tier");
    }

    program.id
}

#[tokio::test]
async fn loyalty_no_program_returns_error() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let result = loyalty_helpers::evaluate_tier(&pool, merchant, customer).await;
    assert!(result.is_err(), "no program should return error");
}

#[tokio::test]
async fn loyalty_no_tiers_returns_no_change() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    loyalty_storage::create_program(
        &pool,
        &CreateProgramRequest {
            merchant_id: merchant,
            name: "Empty".to_string(),
            evaluation_criteria: "spend".to_string(),
            evaluation_period_days: None,
        },
    )
    .await
    .expect("create program");

    let result = loyalty_helpers::evaluate_tier(&pool, merchant, customer)
        .await
        .expect("evaluate");
    assert!(!result.changed);
    assert!(result.new_tier.is_none());
}

#[tokio::test]
async fn loyalty_spend_below_all_tiers_no_assignment() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    create_test_wallet(&pool, merchant, customer).await;

    create_program_with_tiers(
        &pool,
        merchant,
        "spend",
        None,
        vec![("Bronze", 1, 100.0, 1.0), ("Silver", 2, 500.0, 1.5)],
    )
    .await;

    let result = loyalty_helpers::evaluate_tier(&pool, merchant, customer)
        .await
        .expect("evaluate");
    assert!(result.new_tier.is_none());
    assert!(!result.changed);
}

#[tokio::test]
async fn loyalty_spend_qualifies_for_first_tier() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    create_program_with_tiers(
        &pool,
        merchant,
        "spend",
        None,
        vec![("Bronze", 1, 100.0, 1.0), ("Silver", 2, 500.0, 1.5)],
    )
    .await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 150.0),
        unique_key(),
    )
    .await
    .expect("credit 150");

    let result = loyalty_helpers::evaluate_tier(&pool, merchant, customer)
        .await
        .expect("evaluate");
    assert!(result.changed);
    assert_eq!(result.new_tier.as_ref().unwrap().name, "Bronze");
    assert_eq!(result.direction.as_deref(), Some("upgrade"));
}

#[tokio::test]
async fn loyalty_spend_exact_boundary_qualifies() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    create_program_with_tiers(
        &pool,
        merchant,
        "spend",
        None,
        vec![("Bronze", 1, 100.0, 1.0), ("Silver", 2, 500.0, 1.5)],
    )
    .await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 500.0),
        unique_key(),
    )
    .await
    .expect("credit exactly 500");

    let result = loyalty_helpers::evaluate_tier(&pool, merchant, customer)
        .await
        .expect("evaluate");
    assert!(result.changed);
    assert_eq!(result.new_tier.as_ref().unwrap().name, "Silver");
}

#[tokio::test]
async fn loyalty_same_tier_no_change() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    create_program_with_tiers(
        &pool,
        merchant,
        "spend",
        None,
        vec![("Bronze", 1, 100.0, 1.0), ("Silver", 2, 500.0, 1.5)],
    )
    .await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 200.0),
        unique_key(),
    )
    .await
    .expect("credit 200");

    loyalty_helpers::evaluate_tier(&pool, merchant, customer)
        .await
        .expect("first evaluate");

    let result = loyalty_helpers::evaluate_tier(&pool, merchant, customer)
        .await
        .expect("second evaluate");
    assert!(!result.changed);
}

#[tokio::test]
async fn loyalty_earn_multiplier_returns_tier_multiplier() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    create_program_with_tiers(
        &pool,
        merchant,
        "spend",
        None,
        vec![("Gold", 1, 100.0, 1.5)],
    )
    .await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 200.0),
        unique_key(),
    )
    .await
    .expect("credit");

    loyalty_helpers::evaluate_tier(&pool, merchant, customer)
        .await
        .expect("evaluate");

    let mult = loyalty_helpers::get_earn_multiplier(&pool, merchant, customer)
        .await
        .expect("get multiplier");
    assert!(
        (mult - 1.5).abs() < f64::EPSILON,
        "multiplier should be 1.5, got {mult}"
    );
}

#[tokio::test]
async fn loyalty_earn_multiplier_no_program_returns_one() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let mult = loyalty_helpers::get_earn_multiplier(&pool, merchant, customer)
        .await
        .expect("get multiplier");
    assert!(
        (mult - 1.0).abs() < f64::EPSILON,
        "no program should return 1.0"
    );
}

// ===========================================================================
// Redemption Eligibility Tests
// ===========================================================================

#[tokio::test]
async fn redemption_eligibility_with_balance() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 300.0),
        unique_key(),
    )
    .await
    .expect("credit");

    let ctx = OrderContext {
        order_id: "order-elig-001".to_string(),
        order_amount: 1000.0,
        payment_method: None,
        discount_codes: Vec::new(),
    };

    let elig = redemption_helpers::evaluate_eligibility(&pool, wallet_id, &ctx)
        .await
        .expect("eligibility");

    assert!(
        (elig.total_eligible - 300.0).abs() < f64::EPSILON,
        "eligible should be 300, got {}",
        elig.total_eligible
    );
    assert_eq!(elig.buckets.len(), 1);
}

#[tokio::test]
async fn redemption_eligibility_empty_wallet() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let ctx = OrderContext {
        order_id: "order-elig-002".to_string(),
        order_amount: 1000.0,
        payment_method: None,
        discount_codes: Vec::new(),
    };

    let elig = redemption_helpers::evaluate_eligibility(&pool, wallet_id, &ctx)
        .await
        .expect("eligibility");

    assert!(
        (elig.total_eligible - 0.0).abs() < f64::EPSILON,
        "empty wallet should have 0 eligible"
    );
    assert!(elig.buckets.is_empty());
}

#[tokio::test]
async fn redemption_eligibility_held_not_eligible() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(
        &pool,
        NewLedgerEntry {
            wallet_id,
            bucket_type: BucketType::CodPending,
            movement_type: MovementType::Held,
            earning_unit: 500.0,
            currency_equivalent: 500.0,
            conversion_rate: 1.0,
            event_id: None,
            rule_snapshot_id: None,
            campaign_snapshot_id: None,
            actor_type: ActorType::System,
            actor_id: None,
            payment_reference: None,
            transfer_id: None,
            constraints: json!({}),
            expires_at: None,
        },
        unique_key(),
    )
    .await
    .expect("hold 500");

    let ctx = OrderContext {
        order_id: "order-elig-003".to_string(),
        order_amount: 1000.0,
        payment_method: None,
        discount_codes: Vec::new(),
    };

    let elig = redemption_helpers::evaluate_eligibility(&pool, wallet_id, &ctx)
        .await
        .expect("eligibility");

    assert!(
        (elig.total_eligible - 0.0).abs() < f64::EPSILON,
        "held balance should not be eligible, got {}",
        elig.total_eligible
    );
}

#[tokio::test]
async fn redemption_eligibility_multiple_buckets() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 200.0),
        unique_key(),
    )
    .await
    .expect("earned credit");

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::GoodwillCredit, 100.0),
        unique_key(),
    )
    .await
    .expect("goodwill credit");

    let ctx = OrderContext {
        order_id: "order-elig-004".to_string(),
        order_amount: 1000.0,
        payment_method: None,
        discount_codes: Vec::new(),
    };

    let elig = redemption_helpers::evaluate_eligibility(&pool, wallet_id, &ctx)
        .await
        .expect("eligibility");

    assert!(
        (elig.total_eligible - 300.0).abs() < f64::EPSILON,
        "total eligible should be 300, got {}",
        elig.total_eligible
    );
    assert_eq!(elig.buckets.len(), 2);
}

// ===========================================================================
// Redemption State Machine Tests
// ===========================================================================

#[tokio::test]
async fn compensation_from_failed_state_allowed() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 100.0),
        unique_key(),
    )
    .await
    .expect("credit");

    let redemption_id = create_test_redemption(
        &pool,
        merchant,
        wallet_id,
        50.0,
        1000.0,
        RedemptionState::Failed,
        Some(50.0),
    )
    .await;

    let result = redemption_helpers::compensate(&pool, redemption_id).await;
    assert!(result.is_ok(), "compensation from Failed should work");
}

#[tokio::test]
async fn compensation_from_initiated_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let redemption_id = create_test_redemption(
        &pool,
        merchant,
        wallet_id,
        50.0,
        1000.0,
        RedemptionState::Initiated,
        None,
    )
    .await;

    let result = redemption_helpers::compensate(&pool, redemption_id).await;
    assert!(result.is_err(), "compensation from Initiated must fail");
}

#[tokio::test]
async fn compensation_from_rejected_state_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let redemption_id = create_test_redemption(
        &pool,
        merchant,
        wallet_id,
        50.0,
        1000.0,
        RedemptionState::Rejected,
        None,
    )
    .await;

    let result = redemption_helpers::compensate(&pool, redemption_id).await;
    assert!(result.is_err(), "compensation from Rejected must fail");
}

#[tokio::test]
async fn compensation_from_completed_state_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let redemption_id = create_test_redemption(
        &pool,
        merchant,
        wallet_id,
        50.0,
        1000.0,
        RedemptionState::Completed,
        Some(50.0),
    )
    .await;

    let result = redemption_helpers::compensate(&pool, redemption_id).await;
    assert!(result.is_err(), "compensation from Completed must fail");
}

#[tokio::test]
async fn compensation_zero_applied_amount_no_entries() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let redemption_id = create_test_redemption(
        &pool,
        merchant,
        wallet_id,
        50.0,
        1000.0,
        RedemptionState::Failed,
        None,
    )
    .await;

    let result = redemption_helpers::compensate(&pool, redemption_id).await;
    assert!(result.is_ok(), "compensation with zero applied should succeed without creating entries");

    let redemption = redemption_storage::get_redemption(&pool, redemption_id)
        .await
        .expect("get redemption");
    assert_eq!(redemption.state, RedemptionState::Compensated);
}

async fn create_test_redemption(
    pool: &PgPool,
    merchant_id: Uuid,
    wallet_id: Uuid,
    requested_amount: f64,
    order_amount: f64,
    state: RedemptionState,
    applied_amount: Option<f64>,
) -> Uuid {
    let order_id = format!("order-rsm-{}", Uuid::new_v4());
    let redemption = redemption_storage::create_redemption(
        pool,
        merchant_id,
        wallet_id,
        &order_id,
        order_amount,
        None,
        requested_amount,
    )
    .await
    .expect("create redemption");

    if state != RedemptionState::Initiated {
        redemption_storage::update_redemption_state(
            pool,
            redemption.id,
            &state,
            None,
            applied_amount,
            None,
            None,
            None,
            None,
        )
        .await
        .expect("update state");
    }

    redemption.id
}

// ===========================================================================
// Spin Wheel Tests
// ===========================================================================

#[tokio::test]
async fn spin_wheel_no_config_returns_error() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let result = earn_helpers::spin_wheel(
        &pool,
        SpinRequest {
            merchant_id: merchant,
            customer_id: customer,
        },
    )
    .await;

    assert!(result.is_err(), "spin without config must fail");
}

#[tokio::test]
async fn spin_wheel_inactive_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let config = earn_storage::create_wheel_config(&pool, merchant, "Test Wheel", 3).await.expect("wheel");
    sqlx::query("UPDATE spin_wheel_configs SET is_active = false WHERE id = $1")
        .bind(config.id)
        .execute(&pool)
        .await
        .expect("deactivate");

    earn_storage::create_wheel_segment(&pool, config.id, "Win 10", 10.0, 1.0, "#ff0000", 0)
        .await
        .expect("segment");

    let result = earn_helpers::spin_wheel(
        &pool,
        SpinRequest {
            merchant_id: merchant,
            customer_id: customer,
        },
    )
    .await;

    assert!(result.is_err(), "inactive wheel must fail");
}

#[tokio::test]
async fn spin_wheel_daily_limit_enforced() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let wheel = earn_helpers::create_wheel(
        &pool,
        CreateWheelRequest {
            merchant_id: merchant,
            name: Some("Limit Wheel".to_string()),
            daily_spin_limit: Some(2),
            segments: vec![
                CreateSegmentRequest {
                    label: "Win 10".to_string(),
                    reward_amount: 10.0,
                    probability: 1.0,
                    color: None,
                },
            ],
        },
    )
    .await
    .expect("create wheel");

    let spin1 = earn_helpers::spin_wheel(&pool, SpinRequest { merchant_id: merchant, customer_id: customer }).await.expect("spin 1");
    assert_eq!(spin1.spins_remaining_today, 1);

    let spin2 = earn_helpers::spin_wheel(&pool, SpinRequest { merchant_id: merchant, customer_id: customer }).await.expect("spin 2");
    assert_eq!(spin2.spins_remaining_today, 0);

    let spin3 = earn_helpers::spin_wheel(&pool, SpinRequest { merchant_id: merchant, customer_id: customer }).await;
    assert!(spin3.is_err(), "third spin must be rejected (limit=2)");
}

#[tokio::test]
async fn spin_wheel_reward_creates_ledger_entry() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    create_test_wallet(&pool, merchant, customer).await;

    earn_helpers::create_wheel(
        &pool,
        CreateWheelRequest {
            merchant_id: merchant,
            name: Some("Reward Wheel".to_string()),
            daily_spin_limit: Some(5),
            segments: vec![
                CreateSegmentRequest {
                    label: "Win 50".to_string(),
                    reward_amount: 50.0,
                    probability: 1.0,
                    color: None,
                },
            ],
        },
    )
    .await
    .expect("create wheel");

    let result = earn_helpers::spin_wheel(
        &pool,
        SpinRequest {
            merchant_id: merchant,
            customer_id: customer,
        },
    )
    .await
    .expect("spin");

    assert!(result.ledger_entry_id.is_some(), "reward > 0 must create entry");
    assert!((result.reward_amount - 50.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn spin_wheel_zero_reward_no_ledger_entry() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    earn_helpers::create_wheel(
        &pool,
        CreateWheelRequest {
            merchant_id: merchant,
            name: Some("No Prize Wheel".to_string()),
            daily_spin_limit: Some(5),
            segments: vec![
                CreateSegmentRequest {
                    label: "Better Luck".to_string(),
                    reward_amount: 0.0,
                    probability: 1.0,
                    color: None,
                },
            ],
        },
    )
    .await
    .expect("create wheel");

    let result = earn_helpers::spin_wheel(
        &pool,
        SpinRequest {
            merchant_id: merchant,
            customer_id: customer,
        },
    )
    .await
    .expect("spin");

    assert!(result.ledger_entry_id.is_none(), "zero reward must NOT create entry");
    assert!((result.reward_amount - 0.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn spin_wheel_no_segments_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    earn_storage::create_wheel_config(&pool, merchant, "Empty Wheel", 5)
        .await
        .expect("wheel config");

    let result = earn_helpers::spin_wheel(
        &pool,
        SpinRequest {
            merchant_id: merchant,
            customer_id: customer,
        },
    )
    .await;

    assert!(result.is_err(), "wheel with no segments must fail");
}

// ===========================================================================
// Earn Flow — E2E with Event Ingestion
// ===========================================================================

#[tokio::test]
async fn earn_process_prepaid_order_creates_entry() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    rules_storage::create_rule(
        &pool,
        &CreateRuleRequest {
            merchant_id: merchant,
            rule_type: "reward".to_string(),
            name: "5% cashback".to_string(),
            config: json!({
                "event_type": "order.completed",
                "conditions": [],
                "action": {
                    "bucket_type": "earned_credit",
                    "calculation": "percentage",
                    "value": 5.0,
                    "max_amount": null,
                    "conversion_rate": 1.0,
                    "expiry_days": 90
                }
            }),
        },
    )
    .await
    .expect("create rule");

    let (event, _) = event_storage::store_event(
        &pool,
        &IngestEventRequest {
            merchant_id: merchant,
            event_type: "order.completed".to_string(),
            event_source: "shopify".to_string(),
            external_event_id: format!("order-{}", Uuid::new_v4()),
            payload: json!({
                "id": 12345,
                "order_number": 1001,
                "email": "test@example.com",
                "phone": null,
                "total_price": "2000.00",
                "currency": "INR",
                "financial_status": "paid",
                "gateway": "razorpay",
                "payment_gateway_names": ["razorpay"],
                "customer": {
                    "id": 1,
                    "email": "test@example.com",
                    "phone": "+919876543210",
                    "first_name": "Test",
                    "last_name": "User"
                },
                "line_items": []
            }),
        },
    )
    .await
    .expect("store event");

    let result = earn_helpers::process_earn(&pool, event.id).await.expect("process earn");

    assert!(!result.entries_created.is_empty(), "must create at least one entry");
    assert!(!result.is_cod, "razorpay is not COD");

    let entry = &result.entries_created[0];
    assert!((entry.earning_unit - 100.0).abs() < f64::EPSILON, "5% of 2000 = 100, got {}", entry.earning_unit);
    assert_eq!(entry.bucket_type, "EarnedCredit");
    assert_eq!(entry.movement_type, "In");
}

#[tokio::test]
async fn earn_process_cod_order_creates_held_entry() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    rules_storage::create_rule(
        &pool,
        &CreateRuleRequest {
            merchant_id: merchant,
            rule_type: "reward".to_string(),
            name: "5% COD cashback".to_string(),
            config: json!({
                "event_type": "order.completed",
                "conditions": [],
                "action": {
                    "bucket_type": "earned_credit",
                    "calculation": "percentage",
                    "value": 5.0,
                    "max_amount": null,
                    "conversion_rate": 1.0,
                    "expiry_days": 90
                }
            }),
        },
    )
    .await
    .expect("create rule");

    let (event, _) = event_storage::store_event(
        &pool,
        &IngestEventRequest {
            merchant_id: merchant,
            event_type: "order.completed".to_string(),
            event_source: "shopify".to_string(),
            external_event_id: format!("cod-{}", Uuid::new_v4()),
            payload: json!({
                "id": 12346,
                "order_number": 1002,
                "email": null,
                "phone": null,
                "total_price": "1000.00",
                "currency": "INR",
                "financial_status": "pending",
                "gateway": "Cash on Delivery (COD)",
                "payment_gateway_names": ["Cash on Delivery (COD)"],
                "customer": {
                    "id": 2,
                    "email": null,
                    "phone": "+919876543211",
                    "first_name": "COD",
                    "last_name": "User"
                },
                "line_items": []
            }),
        },
    )
    .await
    .expect("store event");

    let result = earn_helpers::process_earn(&pool, event.id).await.expect("process earn");

    assert!(result.is_cod, "COD gateway must be detected as COD");
    assert!(!result.entries_created.is_empty());

    let entry = &result.entries_created[0];
    assert_eq!(entry.bucket_type, "CodPending");
    assert_eq!(entry.movement_type, "Held");
}

#[tokio::test]
async fn earn_process_event_wrong_state_rejected() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    let (event, _) = event_storage::store_event(
        &pool,
        &IngestEventRequest {
            merchant_id: merchant,
            event_type: "order.completed".to_string(),
            event_source: "shopify".to_string(),
            external_event_id: format!("dup-{}", Uuid::new_v4()),
            payload: json!({
                "id": 99999,
                "order_number": 9999,
                "email": null,
                "phone": null,
                "total_price": "100.00",
                "currency": "INR",
                "financial_status": "paid",
                "gateway": "razorpay",
                "payment_gateway_names": ["razorpay"],
                "customer": {
                    "id": 99,
                    "email": null,
                    "phone": "+919999999999",
                    "first_name": null,
                    "last_name": null
                },
                "line_items": []
            }),
        },
    )
    .await
    .expect("store event");

    earn_helpers::process_earn(&pool, event.id).await.expect("first process");

    let result = earn_helpers::process_earn(&pool, event.id).await;
    assert!(result.is_err(), "processing already-processed event must fail");
}

#[tokio::test]
async fn earn_process_no_matching_rules_zero_entries() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    let (event, _) = event_storage::store_event(
        &pool,
        &IngestEventRequest {
            merchant_id: merchant,
            event_type: "order.completed".to_string(),
            event_source: "shopify".to_string(),
            external_event_id: format!("norule-{}", Uuid::new_v4()),
            payload: json!({
                "id": 55555,
                "order_number": 5555,
                "email": null,
                "phone": null,
                "total_price": "500.00",
                "currency": "INR",
                "financial_status": "paid",
                "gateway": "razorpay",
                "payment_gateway_names": ["razorpay"],
                "customer": {
                    "id": 55,
                    "email": null,
                    "phone": "+919555555555",
                    "first_name": null,
                    "last_name": null
                },
                "line_items": []
            }),
        },
    )
    .await
    .expect("store event");

    let result = earn_helpers::process_earn(&pool, event.id).await.expect("process earn");
    assert!(result.entries_created.is_empty(), "no rules means no entries");
}

// ===========================================================================
// Milestone Tests
// ===========================================================================

#[tokio::test]
async fn milestone_order_count_achieved() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    create_test_wallet(&pool, merchant, customer).await;

    earn_storage::create_milestone_config(
        &pool,
        &CreateMilestoneRequest {
            merchant_id: merchant,
            name: "5th Order Bonus".to_string(),
            milestone_type: "order_count".to_string(),
            threshold: 5.0,
            reward_amount: 100.0,
        },
    )
    .await
    .expect("create milestone");

    for i in 0..5 {
        earn_storage::update_order_stats(&pool, merchant, customer, 200.0 * (i as f64 + 1.0))
            .await
            .expect("update stats");
    }

    let result = earn_helpers::check_and_award_milestones(&pool, merchant, customer)
        .await
        .expect("check milestones");

    assert_eq!(result.milestones_achieved.len(), 1);
    assert_eq!(result.milestones_achieved[0].milestone_name, "5th Order Bonus");
    assert!((result.milestones_achieved[0].reward_amount - 100.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn milestone_not_yet_reached() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    create_test_wallet(&pool, merchant, customer).await;

    earn_storage::create_milestone_config(
        &pool,
        &CreateMilestoneRequest {
            merchant_id: merchant,
            name: "10th Order Bonus".to_string(),
            milestone_type: "order_count".to_string(),
            threshold: 10.0,
            reward_amount: 200.0,
        },
    )
    .await
    .expect("create milestone");

    for _ in 0..3 {
        earn_storage::update_order_stats(&pool, merchant, customer, 100.0)
            .await
            .expect("update stats");
    }

    let result = earn_helpers::check_and_award_milestones(&pool, merchant, customer)
        .await
        .expect("check milestones");

    assert!(result.milestones_achieved.is_empty(), "3 orders < 10 threshold");
}

#[tokio::test]
async fn milestone_idempotent_no_double_reward() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    create_test_wallet(&pool, merchant, customer).await;

    earn_storage::create_milestone_config(
        &pool,
        &CreateMilestoneRequest {
            merchant_id: merchant,
            name: "3rd Order".to_string(),
            milestone_type: "order_count".to_string(),
            threshold: 3.0,
            reward_amount: 50.0,
        },
    )
    .await
    .expect("create milestone");

    for _ in 0..3 {
        earn_storage::update_order_stats(&pool, merchant, customer, 100.0)
            .await
            .expect("update stats");
    }

    let first = earn_helpers::check_and_award_milestones(&pool, merchant, customer)
        .await
        .expect("first check");
    assert_eq!(first.milestones_achieved.len(), 1);

    let second = earn_helpers::check_and_award_milestones(&pool, merchant, customer)
        .await
        .expect("second check");
    assert!(second.milestones_achieved.is_empty(), "already achieved, no double reward");
}

#[tokio::test]
async fn milestone_lifetime_spend_achieved() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    create_test_wallet(&pool, merchant, customer).await;

    earn_storage::create_milestone_config(
        &pool,
        &CreateMilestoneRequest {
            merchant_id: merchant,
            name: "VIP Spender".to_string(),
            milestone_type: "lifetime_spend".to_string(),
            threshold: 5000.0,
            reward_amount: 500.0,
        },
    )
    .await
    .expect("create milestone");

    earn_storage::update_order_stats(&pool, merchant, customer, 3000.0).await.expect("order 1");
    earn_storage::update_order_stats(&pool, merchant, customer, 2500.0).await.expect("order 2");

    let result = earn_helpers::check_and_award_milestones(&pool, merchant, customer)
        .await
        .expect("check");

    assert_eq!(result.milestones_achieved.len(), 1);
    assert_eq!(result.milestones_achieved[0].milestone_name, "VIP Spender");
}

// ===========================================================================
// Membership & Multiplier Tests
// ===========================================================================

#[tokio::test]
async fn membership_assign_and_check_status() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let program_id = create_program_with_tiers(
        &pool,
        merchant,
        "spend",
        None,
        vec![("Gold", 1, 0.0, 2.0)],
    )
    .await;

    let tiers = loyalty_storage::get_tiers(&pool, program_id).await.expect("get tiers");
    let gold_tier = &tiers[0];

    let result = earn_helpers::assign_membership(
        &pool,
        AssignMembershipRequest {
            merchant_id: merchant,
            customer_id: customer,
            tier_id: gold_tier.id,
        },
    )
    .await
    .expect("assign membership");

    assert!(result.is_new);
    assert_eq!(result.tier_name, "Gold");
    assert!((result.earn_rate_multiplier - 2.0).abs() < f64::EPSILON);

    let status = earn_helpers::get_membership_status(&pool, merchant, customer)
        .await
        .expect("status");
    assert!(status.is_active);
    assert!((status.earn_rate_multiplier - 2.0).abs() < f64::EPSILON);
    assert!(status.days_remaining > 0);
}

#[tokio::test]
async fn membership_duplicate_assign_returns_existing() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let program_id = create_program_with_tiers(
        &pool,
        merchant,
        "spend",
        None,
        vec![("Silver", 1, 0.0, 1.5)],
    )
    .await;

    let tiers = loyalty_storage::get_tiers(&pool, program_id).await.expect("tiers");
    let req = AssignMembershipRequest {
        merchant_id: merchant,
        customer_id: customer,
        tier_id: tiers[0].id,
    };

    let first = earn_helpers::assign_membership(&pool, req).await.expect("first assign");
    assert!(first.is_new);

    let req2 = AssignMembershipRequest {
        merchant_id: merchant,
        customer_id: customer,
        tier_id: tiers[0].id,
    };
    let second = earn_helpers::assign_membership(&pool, req2).await.expect("second assign");
    assert!(!second.is_new, "same tier must return existing");
}

#[tokio::test]
async fn membership_cancel() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let program_id = create_program_with_tiers(
        &pool,
        merchant,
        "spend",
        None,
        vec![("Bronze", 1, 0.0, 1.2)],
    )
    .await;

    let tiers = loyalty_storage::get_tiers(&pool, program_id).await.expect("tiers");
    let result = earn_helpers::assign_membership(
        &pool,
        AssignMembershipRequest {
            merchant_id: merchant,
            customer_id: customer,
            tier_id: tiers[0].id,
        },
    )
    .await
    .expect("assign");

    let cancelled = earn_helpers::cancel_membership_by_id(&pool, result.membership.id)
        .await
        .expect("cancel");
    assert_eq!(cancelled.status, "cancelled");

    let status = earn_helpers::get_membership_status(&pool, merchant, customer)
        .await
        .expect("status");
    assert!(!status.is_active);
    assert!((status.earn_rate_multiplier - 1.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn membership_multiplier_applied_in_earn() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    let program_id = create_program_with_tiers(
        &pool,
        merchant,
        "spend",
        None,
        vec![("Premium", 1, 0.0, 2.0)],
    )
    .await;

    let tiers = loyalty_storage::get_tiers(&pool, program_id).await.expect("tiers");

    rules_storage::create_rule(
        &pool,
        &CreateRuleRequest {
            merchant_id: merchant,
            rule_type: "reward".to_string(),
            name: "10% cashback".to_string(),
            config: json!({
                "event_type": "order.completed",
                "conditions": [],
                "action": {
                    "bucket_type": "earned_credit",
                    "calculation": "percentage",
                    "value": 10.0,
                    "max_amount": null,
                    "conversion_rate": 1.0,
                    "expiry_days": 90
                }
            }),
        },
    )
    .await
    .expect("create rule");

    let (event, _) = event_storage::store_event(
        &pool,
        &IngestEventRequest {
            merchant_id: merchant,
            event_type: "order.completed".to_string(),
            event_source: "shopify".to_string(),
            external_event_id: format!("mult-{}", Uuid::new_v4()),
            payload: json!({
                "id": 77777,
                "order_number": 7777,
                "email": null,
                "phone": null,
                "total_price": "1000.00",
                "currency": "INR",
                "financial_status": "paid",
                "gateway": "razorpay",
                "payment_gateway_names": ["razorpay"],
                "customer": {
                    "id": 77,
                    "email": null,
                    "phone": "+919777777777",
                    "first_name": "Premium",
                    "last_name": "Member"
                },
                "line_items": []
            }),
        },
    )
    .await
    .expect("store event");

    let result_without = earn_helpers::process_earn(&pool, event.id).await.expect("process");
    let base_earning = result_without.entries_created[0].earning_unit;
    assert!(
        (base_earning - 100.0).abs() < f64::EPSILON,
        "10% of 1000 = 100 (no membership yet), got {base_earning}"
    );

    let customer = result_without.customer_id;
    earn_helpers::assign_membership(
        &pool,
        AssignMembershipRequest {
            merchant_id: merchant,
            customer_id: customer,
            tier_id: tiers[0].id,
        },
    )
    .await
    .expect("assign membership");

    let (event2, _) = event_storage::store_event(
        &pool,
        &IngestEventRequest {
            merchant_id: merchant,
            event_type: "order.completed".to_string(),
            event_source: "shopify".to_string(),
            external_event_id: format!("mult2-{}", Uuid::new_v4()),
            payload: json!({
                "id": 77778,
                "order_number": 7778,
                "email": null,
                "phone": null,
                "total_price": "1000.00",
                "currency": "INR",
                "financial_status": "paid",
                "gateway": "razorpay",
                "payment_gateway_names": ["razorpay"],
                "customer": {
                    "id": 77,
                    "email": null,
                    "phone": "+919777777777",
                    "first_name": "Premium",
                    "last_name": "Member"
                },
                "line_items": []
            }),
        },
    )
    .await
    .expect("store event 2");

    let result_with = earn_helpers::process_earn(&pool, event2.id).await.expect("process 2");
    let boosted_earning = result_with.entries_created[0].earning_unit;
    assert!(
        (boosted_earning - 200.0).abs() < f64::EPSILON,
        "10% of 1000 = 100, x2.0 membership = 200, got {boosted_earning}"
    );
}

// ===========================================================================
// Manual Credit Tests
// ===========================================================================

#[tokio::test]
async fn manual_credit_creates_entry() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let result = earn_helpers::process_manual_credit(
        &pool,
        ManualCreditRequest {
            merchant_id: merchant,
            customer_id: customer,
            amount: 250.0,
            bucket_type: "goodwill_credit".to_string(),
            reason: "customer complaint resolution".to_string(),
            actor_id: "admin-user-1".to_string(),
        },
    )
    .await
    .expect("manual credit");

    assert!((result.amount - 250.0).abs() < f64::EPSILON);

    let balance = ledger_storage::get_balance(&pool, result.wallet_id)
        .await
        .expect("balance");
    assert!(
        (balance.spendable_balance - 250.0).abs() < f64::EPSILON,
        "manual credit should appear in balance, got {}",
        balance.spendable_balance
    );
}

#[tokio::test]
async fn manual_credit_unknown_bucket_defaults_to_goodwill() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;

    let result = earn_helpers::process_manual_credit(
        &pool,
        ManualCreditRequest {
            merchant_id: merchant,
            customer_id: customer,
            amount: 50.0,
            bucket_type: "nonexistent_bucket".to_string(),
            reason: "test".to_string(),
            actor_id: "admin".to_string(),
        },
    )
    .await
    .expect("manual credit with unknown bucket");

    let balance = ledger_storage::get_balance(&pool, result.wallet_id)
        .await
        .expect("balance");

    let goodwill = balance
        .buckets
        .iter()
        .find(|b| b.bucket_type == BucketType::GoodwillCredit);
    assert!(goodwill.is_some(), "unknown bucket should default to GoodwillCredit");
}

// ===========================================================================
// Wallet Policy-Constrained Eligibility Tests
// ===========================================================================

async fn create_wallet_policy(
    pool: &PgPool,
    merchant_id: Uuid,
    bucket_type: &str,
    max_pct: Option<f64>,
    max_fixed: Option<f64>,
    min_redemption: Option<f64>,
    step_size: Option<f64>,
    excluded_pm: &[&str],
    stackable: bool,
) {
    let excluded: Vec<String> = excluded_pm.iter().map(|s| s.to_string()).collect();
    sqlx::query(
        r#"
        INSERT INTO wallet_policies (merchant_id, bucket_type, max_per_order_pct, max_per_order_fixed,
            min_redemption, step_size, excluded_payment_methods, stackable_with_discounts, is_active)
        VALUES ($1, $2::bucket_type, $3, $4, $5, $6, $7, $8, true)
        "#,
    )
    .bind(merchant_id)
    .bind(bucket_type)
    .bind(max_pct)
    .bind(max_fixed)
    .bind(min_redemption)
    .bind(step_size)
    .bind(&excluded)
    .bind(stackable)
    .execute(pool)
    .await
    .expect("create policy");
}

#[tokio::test]
async fn eligibility_respects_max_pct_policy() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 500.0),
        unique_key(),
    )
    .await
    .expect("credit");

    create_wallet_policy(&pool, merchant, "earned_credit", Some(10.0), None, None, None, &[], true).await;

    let ctx = OrderContext {
        order_id: "policy-test-1".to_string(),
        order_amount: 1000.0,
        payment_method: None,
        discount_codes: Vec::new(),
    };

    let elig = redemption_helpers::evaluate_eligibility(&pool, wallet_id, &ctx)
        .await
        .expect("eligibility");

    assert!(
        (elig.total_eligible - 100.0).abs() < f64::EPSILON,
        "10% of 1000 = 100 cap (wallet has 500), got {}",
        elig.total_eligible
    );
}

#[tokio::test]
async fn eligibility_respects_max_fixed_policy() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 500.0),
        unique_key(),
    )
    .await
    .expect("credit");

    create_wallet_policy(&pool, merchant, "earned_credit", None, Some(75.0), None, None, &[], true).await;

    let ctx = OrderContext {
        order_id: "policy-test-2".to_string(),
        order_amount: 1000.0,
        payment_method: None,
        discount_codes: Vec::new(),
    };

    let elig = redemption_helpers::evaluate_eligibility(&pool, wallet_id, &ctx)
        .await
        .expect("eligibility");

    assert!(
        (elig.total_eligible - 75.0).abs() < f64::EPSILON,
        "fixed cap 75, got {}",
        elig.total_eligible
    );
}

#[tokio::test]
async fn eligibility_excluded_payment_method() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 500.0),
        unique_key(),
    )
    .await
    .expect("credit");

    create_wallet_policy(&pool, merchant, "earned_credit", None, None, None, None, &["cod"], true).await;

    let ctx = OrderContext {
        order_id: "policy-test-3".to_string(),
        order_amount: 1000.0,
        payment_method: Some("COD".to_string()),
        discount_codes: Vec::new(),
    };

    let elig = redemption_helpers::evaluate_eligibility(&pool, wallet_id, &ctx)
        .await
        .expect("eligibility");

    assert!(
        (elig.total_eligible - 0.0).abs() < f64::EPSILON,
        "COD excluded, eligible should be 0, got {}",
        elig.total_eligible
    );
}

#[tokio::test]
async fn eligibility_non_excluded_payment_method_passes() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 500.0),
        unique_key(),
    )
    .await
    .expect("credit");

    create_wallet_policy(&pool, merchant, "earned_credit", None, None, None, None, &["cod"], true).await;

    let ctx = OrderContext {
        order_id: "policy-test-4".to_string(),
        order_amount: 1000.0,
        payment_method: Some("UPI".to_string()),
        discount_codes: Vec::new(),
    };

    let elig = redemption_helpers::evaluate_eligibility(&pool, wallet_id, &ctx)
        .await
        .expect("eligibility");

    assert!(
        (elig.total_eligible - 500.0).abs() < f64::EPSILON,
        "UPI not excluded, full 500 eligible, got {}",
        elig.total_eligible
    );
}

#[tokio::test]
async fn eligibility_inactive_policy_treated_as_no_policy() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, BucketType::EarnedCredit, 500.0),
        unique_key(),
    )
    .await
    .expect("credit");

    create_wallet_policy(&pool, merchant, "earned_credit", None, None, None, None, &[], true).await;

    sqlx::query("UPDATE wallet_policies SET is_active = false WHERE merchant_id = $1")
        .bind(merchant)
        .execute(&pool)
        .await
        .expect("deactivate");

    let ctx = OrderContext {
        order_id: "policy-test-5".to_string(),
        order_amount: 1000.0,
        payment_method: None,
        discount_codes: Vec::new(),
    };

    let elig = redemption_helpers::evaluate_eligibility(&pool, wallet_id, &ctx)
        .await
        .expect("eligibility");

    // inactive policy is filtered out by get_wallet_policies (WHERE is_active = true),
    // so it's treated as "no policy" → full spendable amount is eligible
    assert!(
        (elig.total_eligible - 500.0).abs() < f64::EPSILON,
        "inactive policy = no policy = full amount, got {}",
        elig.total_eligible
    );
}

// ===========================================================================
// Streak Tests
// ===========================================================================

#[tokio::test]
async fn streak_not_yet_reached() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    create_test_wallet(&pool, merchant, customer).await;

    earn_storage::create_streak_config(
        &pool,
        &CreateStreakConfigRequest {
            merchant_id: merchant,
            name: "Weekly Warrior".to_string(),
            required_orders: 3,
            window_days: 7,
            reward_amount: 75.0,
        },
    )
    .await
    .expect("create streak");

    let result = earn_helpers::check_and_award_streaks(&pool, merchant, customer)
        .await
        .expect("check streaks");

    assert!(result.streaks_achieved.is_empty(), "0 orders < 3 required");
    assert_eq!(result.active_streaks.len(), 1);
    assert!((result.active_streaks[0].progress_pct - 0.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn streak_achieved_and_rewarded() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    earn_storage::create_streak_config(
        &pool,
        &CreateStreakConfigRequest {
            merchant_id: merchant,
            name: "3-Order Streak".to_string(),
            required_orders: 3,
            window_days: 30,
            reward_amount: 100.0,
        },
    )
    .await
    .expect("create streak");

    for i in 1..=3 {
        ledger_storage::create_entry(
            &pool,
            NewLedgerEntry {
                wallet_id,
                bucket_type: BucketType::EarnedCredit,
                movement_type: MovementType::In,
                earning_unit: 50.0,
                currency_equivalent: 50.0,
                conversion_rate: 1.0,
                event_id: None,
                rule_snapshot_id: None,
                campaign_snapshot_id: None,
                actor_type: ActorType::System,
                actor_id: None,
                payment_reference: Some(format!("order:streak-order-{i}")),
                transfer_id: None,
                constraints: json!({}),
                expires_at: None,
            },
            unique_key(),
        )
        .await
        .expect("create order entry");
    }

    let result = earn_helpers::check_and_award_streaks(&pool, merchant, customer)
        .await
        .expect("check streaks");

    assert_eq!(result.streaks_achieved.len(), 1);
    assert_eq!(result.streaks_achieved[0].streak_name, "3-Order Streak");
    assert!((result.streaks_achieved[0].reward_amount - 100.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn streak_idempotent_no_double_reward() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    earn_storage::create_streak_config(
        &pool,
        &CreateStreakConfigRequest {
            merchant_id: merchant,
            name: "2-Order Streak".to_string(),
            required_orders: 2,
            window_days: 30,
            reward_amount: 50.0,
        },
    )
    .await
    .expect("create streak");

    for i in 1..=2 {
        ledger_storage::create_entry(
            &pool,
            NewLedgerEntry {
                wallet_id,
                bucket_type: BucketType::EarnedCredit,
                movement_type: MovementType::In,
                earning_unit: 50.0,
                currency_equivalent: 50.0,
                conversion_rate: 1.0,
                event_id: None,
                rule_snapshot_id: None,
                campaign_snapshot_id: None,
                actor_type: ActorType::System,
                actor_id: None,
                payment_reference: Some(format!("order:idem-streak-{i}")),
                transfer_id: None,
                constraints: json!({}),
                expires_at: None,
            },
            unique_key(),
        )
        .await
        .expect("entry");
    }

    let first = earn_helpers::check_and_award_streaks(&pool, merchant, customer)
        .await
        .expect("first check");
    assert_eq!(first.streaks_achieved.len(), 1);

    let second = earn_helpers::check_and_award_streaks(&pool, merchant, customer)
        .await
        .expect("second check");
    assert!(second.streaks_achieved.is_empty(), "already rewarded in this window");
}
