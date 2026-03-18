use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use batua::error::AppError;
use batua::services::cod::helpers as cod_helpers;
use batua::services::cod::storage as cod_storage;
use batua::services::events::storage as event_storage;
use batua::services::events::types::IngestEventRequest;
use batua::services::identity::storage as identity_storage;
use batua::services::identity::types::ResolveIdentityRequest;
use batua::services::ledger::storage as ledger_storage;
use batua::services::ledger::types::{
    ActorType, BucketType, CreditState, GetEntriesQuery, MovementType, NewLedgerEntry,
};
use batua::services::rules::helpers as rules_helpers;
use batua::services::rules::storage as rules_storage;
use batua::services::rules::types::{CreateRuleRequest, EvaluationContext};
use batua::services::wallets::storage as wallet_storage;
use batua::services::wallets::types::CreateWalletRequest;

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
    .bind(format!("shop-{}", Uuid::new_v4()))
    .bind("Test Merchant")
    .bind("test.myshopify.com")
    .fetch_one(pool)
    .await
    .expect("create merchant");
    id.0
}

async fn create_test_customer(pool: &PgPool, _label: &str) -> Uuid {
    // Generate a unique valid 10-digit Indian phone number
    let unique = Uuid::new_v4().as_u128() % 9_000_000_000 + 1_000_000_000;
    let phone = format!("{unique}");
    let (customer, _) = identity_storage::resolve_or_create(
        pool,
        &ResolveIdentityRequest {
            phone,
            email: None,
            name: Some("Test User".to_string()),
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

fn make_in_entry(wallet_id: Uuid, amount: f64) -> NewLedgerEntry {
    NewLedgerEntry {
        wallet_id,
        bucket_type: BucketType::EarnedCredit,
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

fn make_held_entry(wallet_id: Uuid, amount: f64) -> NewLedgerEntry {
    NewLedgerEntry {
        wallet_id,
        bucket_type: BucketType::CodPending,
        movement_type: MovementType::Held,
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

fn make_out_entry(wallet_id: Uuid, amount: f64) -> NewLedgerEntry {
    NewLedgerEntry {
        wallet_id,
        bucket_type: BucketType::EarnedCredit,
        movement_type: MovementType::Out,
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

fn unique_key() -> String {
    format!("test-{}", Uuid::new_v4())
}

// ---------------------------------------------------------------------------
// Truth 1 -- Merchant-scoped wallets
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_wallet_is_merchant_scoped() {
    let pool = get_test_pool().await;

    let merchant_a = create_test_merchant(&pool).await;
    let merchant_b = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "10001").await;

    let wallet_a = wallet_storage::get_or_create_wallet(&pool, merchant_a, customer)
        .await
        .expect("wallet A");
    let wallet_b = wallet_storage::get_or_create_wallet(&pool, merchant_b, customer)
        .await
        .expect("wallet B");

    assert_ne!(
        wallet_a.id, wallet_b.id,
        "same customer must get different wallets for different merchants"
    );
    assert_eq!(wallet_a.merchant_id, merchant_a);
    assert_eq!(wallet_b.merchant_id, merchant_b);
    assert_eq!(wallet_a.customer_id, Some(customer));
    assert_eq!(wallet_b.customer_id, Some(customer));
}

#[tokio::test]
async fn test_wallet_unique_per_customer_merchant() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "10002").await;

    let w1 = wallet_storage::get_or_create_wallet(&pool, merchant, customer)
        .await
        .expect("first wallet");
    let w2 = wallet_storage::get_or_create_wallet(&pool, merchant, customer)
        .await
        .expect("second call should return same wallet");

    assert_eq!(
        w1.id, w2.id,
        "get_or_create must return the same wallet for the same customer+merchant"
    );

    let dup = wallet_storage::create_wallet(
        &pool,
        &CreateWalletRequest {
            merchant_id: merchant,
            customer_id: Some(customer),
            is_bearer: false,
            bearer_code: None,
        },
    )
    .await;

    assert!(
        dup.is_err(),
        "explicit duplicate insert must fail with conflict"
    );
    match dup.unwrap_err() {
        AppError::Conflict(_) => {}
        other => panic!("expected Conflict error, got: {other:?}"),
    }
}

#[tokio::test]
async fn test_bearer_wallet() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let bearer_code = format!("GIFT-{}", Uuid::new_v4().as_simple());

    let wallet = wallet_storage::create_wallet(
        &pool,
        &CreateWalletRequest {
            merchant_id: merchant,
            customer_id: None,
            is_bearer: true,
            bearer_code: Some(bearer_code.clone()),
        },
    )
    .await
    .expect("create bearer wallet");

    assert!(wallet.is_bearer, "wallet must be bearer");
    assert_eq!(wallet.customer_id, None, "bearer wallet has no customer_id");
    assert_eq!(
        wallet.bearer_code.as_deref(),
        Some(bearer_code.as_str()),
        "bearer_code must be stored"
    );

    let looked_up = wallet_storage::get_bearer_wallet(&pool, &bearer_code)
        .await
        .expect("lookup")
        .expect("bearer wallet must be found by code");
    assert_eq!(looked_up.id, wallet.id);
}

// ---------------------------------------------------------------------------
// Truth 2 -- Atomic triple
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_entry_stores_atomic_triple() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "20001").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let entry = ledger_storage::create_entry(
        &pool,
        NewLedgerEntry {
            wallet_id,
            bucket_type: BucketType::EarnedCredit,
            movement_type: MovementType::In,
            earning_unit: 100.0,
            currency_equivalent: 50.0,
            conversion_rate: 0.5,
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
    .expect("create entry");

    assert!(
        (entry.earning_unit - 100.0).abs() < f64::EPSILON,
        "earning_unit must be 100"
    );
    assert!(
        (entry.currency_equivalent - 50.0).abs() < f64::EPSILON,
        "currency_equivalent must be 50"
    );
    assert!(
        (entry.conversion_rate - 0.5).abs() < f64::EPSILON,
        "conversion_rate must be 0.5"
    );
}

// ---------------------------------------------------------------------------
// Truth 3 -- Four movements + held not spendable
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_in_movement_increases_balance() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "30001").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 200.0), unique_key())
        .await
        .expect("credit 200");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.displayed_balance - 200.0).abs() < f64::EPSILON,
        "displayed must be 200, got {}",
        balance.displayed_balance
    );
    assert!(
        (balance.spendable_balance - 200.0).abs() < f64::EPSILON,
        "spendable must be 200, got {}",
        balance.spendable_balance
    );
}

#[tokio::test]
async fn test_held_movement_visible_not_spendable() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "30002").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(&pool, make_held_entry(wallet_id, 150.0), unique_key())
        .await
        .expect("hold 150");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.displayed_balance - 150.0).abs() < f64::EPSILON,
        "displayed must include held amount, got {}",
        balance.displayed_balance
    );
    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "spendable must NOT include held amount, got {}",
        balance.spendable_balance
    );
}

#[tokio::test]
async fn test_out_movement_reduces_balance() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "30003").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 500.0), unique_key())
        .await
        .expect("credit 500");
    ledger_storage::create_entry(&pool, make_out_entry(wallet_id, 200.0), unique_key())
        .await
        .expect("debit 200");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.displayed_balance - 300.0).abs() < f64::EPSILON,
        "displayed must be 300, got {}",
        balance.displayed_balance
    );
    assert!(
        (balance.spendable_balance - 300.0).abs() < f64::EPSILON,
        "spendable must be 300, got {}",
        balance.spendable_balance
    );
}

#[tokio::test]
async fn test_across_movement_atomic() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "30004").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    ledger_storage::create_entry(&pool, make_held_entry(wallet_id, 100.0), unique_key())
        .await
        .expect("initial held entry");

    let out_entry = NewLedgerEntry {
        wallet_id,
        bucket_type: BucketType::CodPending,
        movement_type: MovementType::Out,
        earning_unit: 100.0,
        currency_equivalent: 100.0,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some("test-across".to_string()),
        payment_reference: None,
        transfer_id: None,
        constraints: json!({}),
        expires_at: None,
    };
    let in_entry = NewLedgerEntry {
        wallet_id,
        bucket_type: BucketType::EarnedCredit,
        movement_type: MovementType::In,
        earning_unit: 100.0,
        currency_equivalent: 100.0,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: Some("test-across".to_string()),
        payment_reference: None,
        transfer_id: None,
        constraints: json!({}),
        expires_at: None,
    };

    let (out_result, in_result) = ledger_storage::create_across_movement(
        &pool,
        out_entry,
        unique_key(),
        in_entry,
        unique_key(),
    )
    .await
    .expect("across movement");

    assert_eq!(
        out_result.transfer_id, in_result.transfer_id,
        "both entries must share the same transfer_id"
    );
    assert!(
        out_result.transfer_id.is_some(),
        "transfer_id must not be None"
    );
    assert_eq!(out_result.movement_type, MovementType::Out);
    assert_eq!(in_result.movement_type, MovementType::In);
    assert_eq!(out_result.bucket_type, BucketType::CodPending);
    assert_eq!(in_result.bucket_type, BucketType::EarnedCredit);
}

// ---------------------------------------------------------------------------
// Truth 5 -- Books balance
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_displayed_vs_spendable_balance() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "50001").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    // In +500 (EarnedCredit)
    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 500.0), unique_key())
        .await
        .expect("credit 500");

    // Held +200 (CodPending -- visible, not spendable)
    ledger_storage::create_entry(&pool, make_held_entry(wallet_id, 200.0), unique_key())
        .await
        .expect("hold 200");

    // Out -100 (EarnedCredit)
    ledger_storage::create_entry(&pool, make_out_entry(wallet_id, 100.0), unique_key())
        .await
        .expect("debit 100");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    // displayed = (500 - 100) + 200 = 600
    // spendable = (500 - 100) + 0  = 400
    assert!(
        (balance.displayed_balance - 600.0).abs() < f64::EPSILON,
        "displayed must be 600 (In 500 - Out 100 + Held 200), got {}",
        balance.displayed_balance
    );
    assert!(
        (balance.spendable_balance - 400.0).abs() < f64::EPSILON,
        "spendable must be 400 (In 500 - Out 100, Held excluded), got {}",
        balance.spendable_balance
    );

    let earned_bucket = balance
        .buckets
        .iter()
        .find(|b| b.bucket_type == BucketType::EarnedCredit)
        .expect("EarnedCredit bucket");
    assert!(
        (earned_bucket.displayed - 400.0).abs() < f64::EPSILON,
        "EarnedCredit displayed must be 400"
    );
    assert!(
        (earned_bucket.spendable - 400.0).abs() < f64::EPSILON,
        "EarnedCredit spendable must be 400"
    );

    let cod_bucket = balance
        .buckets
        .iter()
        .find(|b| b.bucket_type == BucketType::CodPending)
        .expect("CodPending bucket");
    assert!(
        (cod_bucket.displayed - 200.0).abs() < f64::EPSILON,
        "CodPending displayed must be 200"
    );
    assert!(
        (cod_bucket.spendable - 0.0).abs() < f64::EPSILON,
        "CodPending spendable must be 0"
    );
}

// ---------------------------------------------------------------------------
// Truth 7 -- Time is first-class
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_ledger_immutability() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "70001").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let entry = ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 100.0), unique_key())
        .await
        .expect("create entry");

    let result = sqlx::query("UPDATE ledger_entries SET earning_unit = 999.0 WHERE id = $1")
        .bind(entry.id)
        .execute(&pool)
        .await;

    assert!(
        result.is_err(),
        "UPDATE on earning_unit must be rejected by the immutability trigger"
    );

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("immutable"),
        "error message must mention immutability, got: {err_msg}"
    );

    let unchanged: (f64,) =
        sqlx::query_as("SELECT earning_unit FROM ledger_entries WHERE id = $1")
            .bind(entry.id)
            .fetch_one(&pool)
            .await
            .expect("fetch entry");
    assert!(
        (unchanged.0 - 100.0).abs() < f64::EPSILON,
        "earning_unit must remain 100 after rejected mutation"
    );
}

#[tokio::test]
async fn test_balance_at_historical_point() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "70002").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let t0 = Utc::now();

    // First entry: +100 at t0
    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 100.0), unique_key())
        .await
        .expect("credit 100");

    let t1 = Utc::now() + Duration::milliseconds(10);

    // Brief delay then second entry: +200
    tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 200.0), unique_key())
        .await
        .expect("credit 200");

    // Balance at t1 should only include the first entry
    let historical = ledger_storage::get_balance_at(&pool, wallet_id, t1)
        .await
        .expect("historical balance");
    assert!(
        (historical.displayed_balance - 100.0).abs() < f64::EPSILON,
        "balance at t1 must be 100, got {}",
        historical.displayed_balance
    );

    // Current balance should include both entries
    let current = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("current balance");
    assert!(
        (current.displayed_balance - 300.0).abs() < f64::EPSILON,
        "current balance must be 300, got {}",
        current.displayed_balance
    );

    // Balance at t0 (before any entries) should be 0
    let before_all = ledger_storage::get_balance_at(&pool, wallet_id, t0 - Duration::seconds(1))
        .await
        .expect("balance before all");
    assert!(
        (before_all.displayed_balance - 0.0).abs() < f64::EPSILON,
        "balance before any entries must be 0"
    );
}

// ---------------------------------------------------------------------------
// Truth 8 -- Idempotency
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_idempotent_entry_creation() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "80001").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let idempotency_key = format!("idem-{}", Uuid::new_v4());

    let entry1 = ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, 100.0),
        idempotency_key.clone(),
    )
    .await
    .expect("first call");

    let entry2 = ledger_storage::create_entry(
        &pool,
        make_in_entry(wallet_id, 100.0),
        idempotency_key.clone(),
    )
    .await
    .expect("second call (idempotent)");

    assert_eq!(
        entry1.id, entry2.id,
        "both calls must return the same ledger entry"
    );

    let all = ledger_storage::get_entries(
        &pool,
        wallet_id,
        GetEntriesQuery {
            page: Some(1),
            limit: Some(100),
            bucket_type: None,
            movement_type: None,
        },
    )
    .await
    .expect("get entries");

    let matching: Vec<_> = all
        .iter()
        .filter(|e| e.idempotency_key == idempotency_key)
        .collect();

    assert_eq!(
        matching.len(),
        1,
        "only one entry must exist for the same idempotency key"
    );
}

#[tokio::test]
async fn test_idempotent_event_ingestion() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;

    let req = IngestEventRequest {
        merchant_id: merchant,
        event_type: "order.completed".to_string(),
        event_source: "shopify".to_string(),
        external_event_id: format!("order-{}", Uuid::new_v4()),
        payload: json!({"total_price": "1000.00"}),
    };

    let (event1, is_dup1) = event_storage::store_event(&pool, &req).await.expect("first ingest");
    assert!(!is_dup1, "first ingest must not be a duplicate");

    let (event2, is_dup2) = event_storage::store_event(&pool, &req)
        .await
        .expect("second ingest");
    assert!(is_dup2, "second ingest must return is_duplicate=true");

    assert_eq!(
        event1.id, event2.id,
        "duplicate ingest must return the same event"
    );
}

// ---------------------------------------------------------------------------
// Earn flow
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_earn_prepaid_flow() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "90001").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    // Create a reward rule: 5% on order.completed, EarnedCredit bucket
    let _rule = rules_storage::create_rule(
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

    let context = EvaluationContext {
        merchant_id: merchant,
        event_type: "order.completed".to_string(),
        event_payload: json!({}),
        order_amount: Some(1000.0),
        payment_method: Some("upi".to_string()),
        is_cod: false,
        collections: vec![],
        customer_tags: vec![],
        is_first_order: true,
    };

    let results = rules_helpers::evaluate_rules(&pool, &context)
        .await
        .expect("evaluate rules");

    assert!(!results.is_empty(), "must match at least one rule");

    let eval = &results[0];
    assert!(eval.matched, "rule must match");
    assert!(
        (eval.earning_unit - 50.0).abs() < f64::EPSILON,
        "5% of 1000 = 50, got {}",
        eval.earning_unit
    );

    let entry = ledger_storage::create_entry(
        &pool,
        NewLedgerEntry {
            wallet_id,
            bucket_type: BucketType::EarnedCredit,
            movement_type: MovementType::In,
            earning_unit: eval.earning_unit,
            currency_equivalent: eval.currency_equivalent,
            conversion_rate: eval.conversion_rate,
            event_id: None,
            rule_snapshot_id: eval.rule_snapshot_id,
            campaign_snapshot_id: eval.campaign_snapshot_id,
            actor_type: ActorType::System,
            actor_id: None,
            payment_reference: Some("order:12345".to_string()),
            transfer_id: None,
            constraints: json!({}),
            expires_at: None,
        },
        unique_key(),
    )
    .await
    .expect("create earn entry");

    assert_eq!(entry.movement_type, MovementType::In);
    assert_eq!(entry.bucket_type, BucketType::EarnedCredit);
    assert!(
        (entry.earning_unit - 50.0).abs() < f64::EPSILON,
        "ledger entry earning_unit must be 50"
    );
    assert!(
        entry.rule_snapshot_id.is_some(),
        "must have rule_snapshot_id linking to immutable snapshot"
    );

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");
    assert!(
        (balance.spendable_balance - 50.0).abs() < f64::EPSILON,
        "wallet spendable must be 50"
    );
}

#[tokio::test]
async fn test_earn_cod_flow() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "90002").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    // COD earn creates a Held entry in CodPending bucket
    let held_entry = ledger_storage::create_entry(
        &pool,
        NewLedgerEntry {
            wallet_id,
            bucket_type: BucketType::CodPending,
            movement_type: MovementType::Held,
            earning_unit: 50.0,
            currency_equivalent: 50.0,
            conversion_rate: 1.0,
            event_id: None,
            rule_snapshot_id: None,
            campaign_snapshot_id: None,
            actor_type: ActorType::System,
            actor_id: None,
            payment_reference: Some("order:COD-001".to_string()),
            transfer_id: None,
            constraints: json!({}),
            expires_at: None,
        },
        unique_key(),
    )
    .await
    .expect("create COD held entry");

    assert_eq!(held_entry.movement_type, MovementType::Held);
    assert_eq!(held_entry.bucket_type, BucketType::CodPending);

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");
    assert!(
        (balance.displayed_balance - 50.0).abs() < f64::EPSILON,
        "displayed must show held amount"
    );
    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "spendable must be 0 for COD pending"
    );

    // Create the COD order record tracking this held entry
    let cod_order = cod_storage::create_cod_order(
        &pool,
        merchant,
        "COD-ORDER-001",
        wallet_id,
        held_entry.id,
    )
    .await
    .expect("create cod order");
    assert_eq!(cod_order.state, "pending");
    assert_eq!(cod_order.ledger_entry_id, held_entry.id);
}

// ---------------------------------------------------------------------------
// Redemption flow
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_redemption_reduces_balance() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "60001").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    // Credit 500 into the wallet
    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 500.0), unique_key())
        .await
        .expect("credit 500");

    let balance_before = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance before");
    assert!(
        (balance_before.spendable_balance - 500.0).abs() < f64::EPSILON,
        "spendable must be 500 before redemption"
    );

    // Redeem 200 via an Out movement
    ledger_storage::create_entry(
        &pool,
        NewLedgerEntry {
            wallet_id,
            bucket_type: BucketType::EarnedCredit,
            movement_type: MovementType::Out,
            earning_unit: 200.0,
            currency_equivalent: 200.0,
            conversion_rate: 1.0,
            event_id: None,
            rule_snapshot_id: None,
            campaign_snapshot_id: None,
            actor_type: ActorType::System,
            actor_id: Some("redemption:test".to_string()),
            payment_reference: Some("order:RED-001".to_string()),
            transfer_id: None,
            constraints: json!({"redemption_id": "test-001"}),
            expires_at: None,
        },
        unique_key(),
    )
    .await
    .expect("redeem 200");

    let balance_after = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance after");

    assert!(
        (balance_after.spendable_balance - 300.0).abs() < f64::EPSILON,
        "spendable must be 300 after redeeming 200, got {}",
        balance_after.spendable_balance
    );
    assert!(
        (balance_after.displayed_balance - 300.0).abs() < f64::EPSILON,
        "displayed must also be 300 after redeeming 200"
    );
}

// ---------------------------------------------------------------------------
// COD lifecycle
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_cod_delivery_releases_credit() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "COD01").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    // Step 1: Create the Held entry (simulating earn flow for COD order)
    let held_entry = ledger_storage::create_entry(
        &pool,
        make_held_entry(wallet_id, 75.0),
        unique_key(),
    )
    .await
    .expect("create held entry");

    // Step 2: Create COD order tracking record
    let order_id = format!("SHOP-COD-{}", Uuid::new_v4().as_simple());
    cod_storage::create_cod_order(&pool, merchant, &order_id, wallet_id, held_entry.id)
        .await
        .expect("create cod order");

    // Verify initial state: displayed=75, spendable=0
    let balance_before = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("pre-delivery balance");
    assert!(
        (balance_before.displayed_balance - 75.0).abs() < f64::EPSILON,
        "pre-delivery displayed must be 75"
    );
    assert!(
        (balance_before.spendable_balance - 0.0).abs() < f64::EPSILON,
        "pre-delivery spendable must be 0"
    );

    // Step 3: Process delivery webhook -- this does the across movement
    cod_helpers::process_delivery(&pool, &order_id, merchant)
        .await
        .expect("process delivery");

    // Verify: credit moved from CodPending to EarnedCredit
    let balance_after = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("post-delivery balance");

    // After across: CodPending has Held +75 and Out -75 = 0 displayed
    // EarnedCredit has In +75 = 75 displayed and spendable
    assert!(
        (balance_after.spendable_balance - 75.0).abs() < f64::EPSILON,
        "post-delivery spendable must be 75, got {}",
        balance_after.spendable_balance
    );

    // Verify COD order state updated
    let cod_order = cod_storage::get_cod_order_by_order_id(&pool, merchant, &order_id)
        .await
        .expect("get cod order");
    assert_eq!(
        cod_order.state, "delivered",
        "COD order state must be 'delivered'"
    );
    assert!(
        cod_order.released_entry_id.is_some(),
        "released_entry_id must be set"
    );
}

#[tokio::test]
async fn test_cod_rto_cancels_credit() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "COD02").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let held_entry = ledger_storage::create_entry(
        &pool,
        make_held_entry(wallet_id, 60.0),
        unique_key(),
    )
    .await
    .expect("create held entry");

    let order_id = format!("SHOP-RTO-{}", Uuid::new_v4().as_simple());
    cod_storage::create_cod_order(&pool, merchant, &order_id, wallet_id, held_entry.id)
        .await
        .expect("create cod order");

    // Process RTO -- creates Out entry cancelling the held credit
    cod_helpers::process_rto(&pool, &order_id, merchant)
        .await
        .expect("process rto");

    let balance_after = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("post-rto balance");

    // After RTO: CodPending has Held +60 and Out -60 = 0 displayed, 0 spendable
    assert!(
        (balance_after.displayed_balance - 0.0).abs() < f64::EPSILON,
        "post-RTO displayed must be 0, got {}",
        balance_after.displayed_balance
    );
    assert!(
        (balance_after.spendable_balance - 0.0).abs() < f64::EPSILON,
        "post-RTO spendable must be 0, got {}",
        balance_after.spendable_balance
    );

    let cod_order = cod_storage::get_cod_order_by_order_id(&pool, merchant, &order_id)
        .await
        .expect("get cod order");
    assert_eq!(cod_order.state, "rto", "COD order state must be 'rto'");
    assert!(
        cod_order.cancelled_entry_id.is_some(),
        "cancelled_entry_id must be set"
    );
}

// ---------------------------------------------------------------------------
// Additional safety-net tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_entry_state_defaults_to_active() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "99001").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let entry = ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 42.0), unique_key())
        .await
        .expect("create entry");

    assert_eq!(
        entry.state,
        CreditState::Active,
        "new entries must default to Active state"
    );
}

#[tokio::test]
async fn test_expired_entries_excluded_from_balance() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "99002").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    // Entry that expires in the past
    let expired_entry = NewLedgerEntry {
        wallet_id,
        bucket_type: BucketType::EarnedCredit,
        movement_type: MovementType::In,
        earning_unit: 100.0,
        currency_equivalent: 100.0,
        conversion_rate: 1.0,
        event_id: None,
        rule_snapshot_id: None,
        campaign_snapshot_id: None,
        actor_type: ActorType::System,
        actor_id: None,
        payment_reference: None,
        transfer_id: None,
        constraints: json!({}),
        expires_at: Some(Utc::now() - Duration::hours(1)),
    };

    ledger_storage::create_entry(&pool, expired_entry, unique_key())
        .await
        .expect("create expired entry");

    // Entry with no expiry
    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 50.0), unique_key())
        .await
        .expect("create valid entry");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.displayed_balance - 50.0).abs() < f64::EPSILON,
        "expired entry must be excluded from balance, got {}",
        balance.displayed_balance
    );
}

#[tokio::test]
async fn test_multiple_buckets_balance_independently() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "99003").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    // EarnedCredit: +100
    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 100.0), unique_key())
        .await
        .expect("earned credit");

    // GoodwillCredit: +50
    ledger_storage::create_entry(
        &pool,
        NewLedgerEntry {
            wallet_id,
            bucket_type: BucketType::GoodwillCredit,
            movement_type: MovementType::In,
            earning_unit: 50.0,
            currency_equivalent: 50.0,
            conversion_rate: 1.0,
            event_id: None,
            rule_snapshot_id: None,
            campaign_snapshot_id: None,
            actor_type: ActorType::Human,
            actor_id: Some("admin-user".to_string()),
            payment_reference: None,
            transfer_id: None,
            constraints: json!({}),
            expires_at: None,
        },
        unique_key(),
    )
    .await
    .expect("goodwill credit");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.displayed_balance - 150.0).abs() < f64::EPSILON,
        "total displayed must be 150, got {}",
        balance.displayed_balance
    );
    assert!(
        (balance.spendable_balance - 150.0).abs() < f64::EPSILON,
        "total spendable must be 150"
    );

    assert_eq!(
        balance.buckets.len(),
        2,
        "must have two independent bucket balances"
    );
}

#[tokio::test]
async fn test_ledger_state_update_allowed() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "99004").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let entry = ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 100.0), unique_key())
        .await
        .expect("create entry");

    // The immutability trigger allows state and expires_at updates
    let result =
        sqlx::query("UPDATE ledger_entries SET state = 'redeemed'::credit_state WHERE id = $1")
            .bind(entry.id)
            .execute(&pool)
            .await;

    assert!(
        result.is_ok(),
        "state update must be allowed by the immutability trigger"
    );

    let updated: (String,) = sqlx::query_as(
        "SELECT state::text FROM ledger_entries WHERE id = $1",
    )
    .bind(entry.id)
    .fetch_one(&pool)
    .await
    .expect("fetch updated");

    assert_eq!(updated.0, "redeemed", "state must be updated to redeemed");
}

#[tokio::test]
async fn test_cod_delivery_then_rto_rejected() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "COD03").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    let held_entry = ledger_storage::create_entry(
        &pool,
        make_held_entry(wallet_id, 80.0),
        unique_key(),
    )
    .await
    .expect("create held entry");

    let order_id = format!("SHOP-DOUBLE-{}", Uuid::new_v4().as_simple());
    cod_storage::create_cod_order(&pool, merchant, &order_id, wallet_id, held_entry.id)
        .await
        .expect("create cod order");

    cod_helpers::process_delivery(&pool, &order_id, merchant)
        .await
        .expect("process delivery");

    // After delivery, attempting RTO should fail because state is no longer pending
    let rto_result = cod_helpers::process_rto(&pool, &order_id, merchant).await;
    assert!(
        rto_result.is_err(),
        "RTO after delivery must fail since state is no longer 'pending'"
    );
}

#[tokio::test]
async fn test_rule_evaluation_with_conditions() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;

    // Rule: 10% cashback on UPI orders above 500
    rules_storage::create_rule(
        &pool,
        &CreateRuleRequest {
            merchant_id: merchant,
            rule_type: "reward".to_string(),
            name: "UPI high-value cashback".to_string(),
            config: json!({
                "event_type": "order.completed",
                "conditions": [
                    {"field": "payment_method", "operator": "eq", "value": "upi"},
                    {"field": "order_amount", "operator": "gt", "value": 500.0}
                ],
                "action": {
                    "bucket_type": "earned_credit",
                    "calculation": "percentage",
                    "value": 10.0,
                    "max_amount": 200.0,
                    "conversion_rate": 1.0,
                    "expiry_days": 30
                }
            }),
        },
    )
    .await
    .expect("create rule");

    // Matching context: UPI + 1000
    let matching_ctx = EvaluationContext {
        merchant_id: merchant,
        event_type: "order.completed".to_string(),
        event_payload: json!({}),
        order_amount: Some(1000.0),
        payment_method: Some("upi".to_string()),
        is_cod: false,
        collections: vec![],
        customer_tags: vec![],
        is_first_order: false,
    };

    let results = rules_helpers::evaluate_rules(&pool, &matching_ctx)
        .await
        .expect("evaluate matching");
    assert_eq!(results.len(), 1, "must match exactly one rule");
    assert!(
        (results[0].earning_unit - 100.0).abs() < f64::EPSILON,
        "10% of 1000 = 100, got {}",
        results[0].earning_unit
    );

    // Non-matching context: COD payment
    let non_matching_ctx = EvaluationContext {
        merchant_id: merchant,
        event_type: "order.completed".to_string(),
        event_payload: json!({}),
        order_amount: Some(1000.0),
        payment_method: Some("cod".to_string()),
        is_cod: true,
        collections: vec![],
        customer_tags: vec![],
        is_first_order: false,
    };

    let no_results = rules_helpers::evaluate_rules(&pool, &non_matching_ctx)
        .await
        .expect("evaluate non-matching");
    assert!(
        no_results.is_empty(),
        "COD order must not match UPI-only rule"
    );

    // Test max_amount cap: 10% of 5000 = 500, but capped at 200
    let high_value_ctx = EvaluationContext {
        merchant_id: merchant,
        event_type: "order.completed".to_string(),
        event_payload: json!({}),
        order_amount: Some(5000.0),
        payment_method: Some("upi".to_string()),
        is_cod: false,
        collections: vec![],
        customer_tags: vec![],
        is_first_order: false,
    };

    let capped = rules_helpers::evaluate_rules(&pool, &high_value_ctx)
        .await
        .expect("evaluate capped");
    assert_eq!(capped.len(), 1);
    assert!(
        (capped[0].earning_unit - 200.0).abs() < f64::EPSILON,
        "earning must be capped at 200, got {}",
        capped[0].earning_unit
    );
}

#[tokio::test]
async fn test_customer_resolve_or_create_idempotent() {
    let pool = get_test_pool().await;

    let unique = Uuid::new_v4().as_u128() % 9_000_000_000 + 1_000_000_000;
    let req = ResolveIdentityRequest {
        phone: format!("+91{unique}"),
        email: Some("test@example.com".to_string()),
        name: Some("Test User".to_string()),
        external_id: None,
    };

    let (c1, is_new1) = identity_storage::resolve_or_create(&pool, &req)
        .await
        .expect("first resolve");
    assert!(is_new1, "first resolve must create a new customer");

    let (c2, is_new2) = identity_storage::resolve_or_create(&pool, &req)
        .await
        .expect("second resolve");
    assert!(!is_new2, "second resolve must find existing customer");

    assert_eq!(
        c1.id, c2.id,
        "both resolves must return the same customer"
    );
}

#[tokio::test]
async fn test_full_lifecycle_earn_to_redeem() {
    let pool = get_test_pool().await;

    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool, "LF001").await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;

    // Earn: credit 300
    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 300.0), unique_key())
        .await
        .expect("earn 300");

    let b1 = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance after earn");
    assert!(
        (b1.spendable_balance - 300.0).abs() < f64::EPSILON,
        "spendable after earn"
    );

    // Redeem: debit 120
    ledger_storage::create_entry(&pool, make_out_entry(wallet_id, 120.0), unique_key())
        .await
        .expect("redeem 120");

    let b2 = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance after redeem");
    assert!(
        (b2.spendable_balance - 180.0).abs() < f64::EPSILON,
        "spendable after redeem must be 180, got {}",
        b2.spendable_balance
    );

    // Earn more: credit 50
    ledger_storage::create_entry(&pool, make_in_entry(wallet_id, 50.0), unique_key())
        .await
        .expect("earn 50 more");

    let b3 = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("final balance");
    assert!(
        (b3.spendable_balance - 230.0).abs() < f64::EPSILON,
        "final spendable must be 230, got {}",
        b3.spendable_balance
    );

    // Verify all entries are tracked
    let entries = ledger_storage::get_entries(
        &pool,
        wallet_id,
        GetEntriesQuery {
            page: Some(1),
            limit: Some(100),
            bucket_type: None,
            movement_type: None,
        },
    )
    .await
    .expect("get all entries");
    assert_eq!(entries.len(), 3, "must have 3 entries total");
}
