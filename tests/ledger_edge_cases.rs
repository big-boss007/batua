use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use batua::services::cod::helpers as cod_helpers;
use batua::services::cod::storage as cod_storage;
use batua::services::identity::storage as identity_storage;
use batua::services::identity::types::ResolveIdentityRequest;
use batua::services::ledger::storage as ledger_storage;
use batua::services::ledger::types::{
    ActorType, BucketType, GetEntriesQuery, MovementType, NewLedgerEntry,
};
use batua::services::wallets::storage as wallet_storage;

// ---------------------------------------------------------------------------
// Test infrastructure (mirrors integration_tests.rs)
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
    .bind(format!("edge-{}", Uuid::new_v4()))
    .bind("Edge Test Merchant")
    .bind("edge.myshopify.com")
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
            name: Some("Edge Test User".to_string()),
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

async fn setup() -> (PgPool, Uuid, Uuid) {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;
    let customer = create_test_customer(&pool).await;
    let wallet_id = create_test_wallet(&pool, merchant, customer).await;
    (pool, merchant, wallet_id)
}

fn make_entry(
    wallet_id: Uuid,
    bucket: BucketType,
    movement: MovementType,
    amount: f64,
) -> NewLedgerEntry {
    NewLedgerEntry {
        wallet_id,
        bucket_type: bucket,
        movement_type: movement,
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
    format!("edge-{}", Uuid::new_v4())
}

// ===========================================================================
// Edge Case 1: Overdraw — Out exceeds In
// The system has no overdraft protection at the ledger layer.
// Balance should clamp to 0, not go negative.
// ===========================================================================

#[tokio::test]
async fn test_overdraw_balance_clamps_to_zero() {
    let (pool, _, wallet_id) = setup().await;

    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0),
        unique_key(),
    )
    .await
    .expect("credit 100");

    // Debit MORE than available
    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::Out, 250.0),
        unique_key(),
    )
    .await
    .expect("debit 250 — system allows this");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    // Raw math: 100 - 250 = -150, but build_wallet_balance clamps to 0
    assert!(
        balance.spendable_balance >= 0.0,
        "spendable must not be negative, got {}",
        balance.spendable_balance
    );
    assert!(
        balance.displayed_balance >= 0.0,
        "displayed must not be negative, got {}",
        balance.displayed_balance
    );
    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "overdraw should clamp spendable to 0, got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 2: Zero amount entries
// ===========================================================================

#[tokio::test]
async fn test_zero_amount_entry_accepted() {
    let (pool, _, wallet_id) = setup().await;

    let entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 0.0),
        unique_key(),
    )
    .await
    .expect("zero amount entry should succeed");

    assert!(
        (entry.earning_unit - 0.0).abs() < f64::EPSILON,
        "earning_unit must be 0"
    );

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");
    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "balance should remain 0 after zero-amount entry"
    );
}

// ===========================================================================
// Edge Case 3: Negative earning_unit
// The system does NOT validate against negative amounts.
// ===========================================================================

#[tokio::test]
async fn test_negative_earning_unit_accepted() {
    let (pool, _, wallet_id) = setup().await;

    // A negative In movement — economically nonsensical, but no validation stops it
    let entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, -50.0),
        unique_key(),
    )
    .await
    .expect("negative amount entry — system has no validation");

    assert!(
        (entry.earning_unit - (-50.0)).abs() < f64::EPSILON,
        "negative earning_unit was stored as-is"
    );

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    // In with -50 means displayed += -50, spendable += -50 → both clamped to 0
    assert!(
        balance.spendable_balance >= 0.0,
        "negative In should still clamp to 0, got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 4: Very large amounts — f64 precision boundary
// ===========================================================================

#[tokio::test]
async fn test_large_amount_precision() {
    let (pool, _, wallet_id) = setup().await;

    // 2^53 - 1 is the max safe integer for f64
    let large_amount: f64 = 9_007_199_254_740_992.0;

    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, large_amount),
        unique_key(),
    )
    .await
    .expect("large amount entry");

    // Add a small amount to test precision loss
    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 1.0),
        unique_key(),
    )
    .await
    .expect("small increment");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    // At this scale, f64 cannot distinguish large_amount vs large_amount+1
    // This test documents the precision limitation
    assert!(
        balance.spendable_balance >= large_amount,
        "large balance should be at least the big amount, got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 5: Conversion rate edge cases (zero, negative)
// ===========================================================================

#[tokio::test]
async fn test_zero_conversion_rate_accepted() {
    let (pool, _, wallet_id) = setup().await;

    let mut entry = make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0);
    entry.conversion_rate = 0.0;

    let result = ledger_storage::create_entry(&pool, entry, unique_key()).await;
    // System allows conversion_rate=0 since there's no validation
    assert!(
        result.is_ok(),
        "conversion_rate=0 is accepted (no validation)"
    );
    assert!(
        (result.unwrap().conversion_rate - 0.0).abs() < f64::EPSILON,
        "conversion_rate 0 stored as-is"
    );
}

#[tokio::test]
async fn test_negative_conversion_rate_accepted() {
    let (pool, _, wallet_id) = setup().await;

    let mut entry = make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0);
    entry.conversion_rate = -1.0;

    let result = ledger_storage::create_entry(&pool, entry, unique_key()).await;
    assert!(
        result.is_ok(),
        "negative conversion_rate is accepted (no validation)"
    );
}

// ===========================================================================
// Edge Case 6: Expires-in-the-past entry
// Entry created with expires_at already in the past — immediately invisible
// ===========================================================================

#[tokio::test]
async fn test_entry_with_past_expiry_immediately_invisible() {
    let (pool, _, wallet_id) = setup().await;

    let mut entry = make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 500.0);
    entry.expires_at = Some(Utc::now() - Duration::seconds(10));

    ledger_storage::create_entry(&pool, entry, unique_key())
        .await
        .expect("create already-expired entry");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "already-expired entry must not appear in balance, got {}",
        balance.spendable_balance
    );
    assert!(
        (balance.displayed_balance - 0.0).abs() < f64::EPSILON,
        "already-expired entry must not appear in displayed balance"
    );
}

// ===========================================================================
// Edge Case 7: Expiry exactly at now() boundary
// ===========================================================================

#[tokio::test]
async fn test_entry_expiring_soon_visible_until_expiry() {
    let (pool, _, wallet_id) = setup().await;

    let future_expiry = Utc::now() + Duration::hours(1);
    let mut entry = make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 200.0);
    entry.expires_at = Some(future_expiry);

    ledger_storage::create_entry(&pool, entry, unique_key())
        .await
        .expect("create entry with future expiry");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");
    assert!(
        (balance.spendable_balance - 200.0).abs() < f64::EPSILON,
        "entry with future expiry should be visible, got {}",
        balance.spendable_balance
    );

    // Historical query AFTER expiry should exclude it
    let after_expiry = future_expiry + Duration::seconds(1);
    let historical = ledger_storage::get_balance_at(&pool, wallet_id, after_expiry)
        .await
        .expect("historical balance after expiry");
    assert!(
        (historical.spendable_balance - 0.0).abs() < f64::EPSILON,
        "balance after expiry should be 0, got {}",
        historical.spendable_balance
    );
}

// ===========================================================================
// Edge Case 8: State change excludes from balance
// Marking entry as 'cancelled' should remove it from balance calculation
// ===========================================================================

#[tokio::test]
async fn test_cancelled_entry_excluded_from_balance() {
    let (pool, _, wallet_id) = setup().await;

    let entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 300.0),
        unique_key(),
    )
    .await
    .expect("create entry");

    let balance_before = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance before");
    assert!(
        (balance_before.spendable_balance - 300.0).abs() < f64::EPSILON,
        "balance before cancel"
    );

    // Cancel the entry
    sqlx::query("UPDATE ledger_entries SET state = 'cancelled'::credit_state WHERE id = $1")
        .bind(entry.id)
        .execute(&pool)
        .await
        .expect("cancel entry");

    let balance_after = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance after cancel");
    assert!(
        (balance_after.spendable_balance - 0.0).abs() < f64::EPSILON,
        "cancelled entry must be excluded from balance, got {}",
        balance_after.spendable_balance
    );
}

#[tokio::test]
async fn test_reversed_entry_excluded_from_balance() {
    let (pool, _, wallet_id) = setup().await;

    let entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 400.0),
        unique_key(),
    )
    .await
    .expect("create entry");

    sqlx::query("UPDATE ledger_entries SET state = 'reversed'::credit_state WHERE id = $1")
        .bind(entry.id)
        .execute(&pool)
        .await
        .expect("reverse entry");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");
    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "reversed entry must be excluded, got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 9: Immutability trigger covers all protected fields
// ===========================================================================

#[tokio::test]
async fn test_immutability_rejects_currency_equivalent_update() {
    let (pool, _, wallet_id) = setup().await;

    let entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0),
        unique_key(),
    )
    .await
    .expect("create entry");

    let result =
        sqlx::query("UPDATE ledger_entries SET currency_equivalent = 999.0 WHERE id = $1")
            .bind(entry.id)
            .execute(&pool)
            .await;

    assert!(result.is_err(), "currency_equivalent update must be rejected");
}

#[tokio::test]
async fn test_immutability_rejects_bucket_type_update() {
    let (pool, _, wallet_id) = setup().await;

    let entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0),
        unique_key(),
    )
    .await
    .expect("create entry");

    let result = sqlx::query(
        "UPDATE ledger_entries SET bucket_type = 'gift_card'::bucket_type WHERE id = $1",
    )
    .bind(entry.id)
    .execute(&pool)
    .await;

    assert!(result.is_err(), "bucket_type update must be rejected");
}

#[tokio::test]
async fn test_immutability_rejects_movement_type_update() {
    let (pool, _, wallet_id) = setup().await;

    let entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0),
        unique_key(),
    )
    .await
    .expect("create entry");

    let result = sqlx::query(
        "UPDATE ledger_entries SET movement_type = 'out'::movement_type WHERE id = $1",
    )
    .bind(entry.id)
    .execute(&pool)
    .await;

    assert!(result.is_err(), "movement_type update must be rejected");
}

#[tokio::test]
async fn test_immutability_rejects_wallet_id_update() {
    let (pool, _, wallet_id) = setup().await;

    let entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0),
        unique_key(),
    )
    .await
    .expect("create entry");

    let result = sqlx::query("UPDATE ledger_entries SET wallet_id = $1 WHERE id = $2")
        .bind(Uuid::new_v4())
        .bind(entry.id)
        .execute(&pool)
        .await;

    assert!(result.is_err(), "wallet_id update must be rejected");
}

#[tokio::test]
async fn test_immutability_allows_expires_at_update() {
    let (pool, _, wallet_id) = setup().await;

    let entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0),
        unique_key(),
    )
    .await
    .expect("create entry");

    let new_expiry = Utc::now() + Duration::days(30);
    let result = sqlx::query("UPDATE ledger_entries SET expires_at = $1 WHERE id = $2")
        .bind(new_expiry)
        .bind(entry.id)
        .execute(&pool)
        .await;

    assert!(
        result.is_ok(),
        "expires_at update must be allowed, got {:?}",
        result.err()
    );
}

// ===========================================================================
// Edge Case 10: Empty wallet balance
// ===========================================================================

#[tokio::test]
async fn test_empty_wallet_has_zero_balance() {
    let (pool, _, wallet_id) = setup().await;

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "empty wallet spendable must be 0"
    );
    assert!(
        (balance.displayed_balance - 0.0).abs() < f64::EPSILON,
        "empty wallet displayed must be 0"
    );
    assert!(
        balance.buckets.is_empty(),
        "empty wallet must have no bucket entries"
    );
}

// ===========================================================================
// Edge Case 11: All 8 bucket types tracked independently
// ===========================================================================

#[tokio::test]
async fn test_all_bucket_types_independent() {
    let (pool, _, wallet_id) = setup().await;

    let buckets = vec![
        BucketType::EarnedCredit,
        BucketType::CodPending,
        BucketType::GiftCard,
        BucketType::CustomerFunded,
        BucketType::ReferralReward,
        BucketType::GoodwillCredit,
        BucketType::MembershipBenefit,
        BucketType::RefundCredit,
    ];

    for (i, bucket) in buckets.iter().enumerate() {
        let amount = (i as f64 + 1.0) * 10.0; // 10, 20, 30, ...
        ledger_storage::create_entry(
            &pool,
            make_entry(wallet_id, bucket.clone(), MovementType::In, amount),
            unique_key(),
        )
        .await
        .unwrap_or_else(|_| panic!("credit {amount} to {bucket:?}"));
    }

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    // Total: 10 + 20 + 30 + 40 + 50 + 60 + 70 + 80 = 360
    assert!(
        (balance.displayed_balance - 360.0).abs() < f64::EPSILON,
        "total displayed must be 360, got {}",
        balance.displayed_balance
    );
    assert_eq!(
        balance.buckets.len(),
        8,
        "must have 8 independent bucket balances, got {}",
        balance.buckets.len()
    );
}

// ===========================================================================
// Edge Case 12: Held entries in non-CodPending buckets
// ===========================================================================

#[tokio::test]
async fn test_held_in_non_cod_bucket_still_not_spendable() {
    let (pool, _, wallet_id) = setup().await;

    // Held movement in EarnedCredit — unusual but the system allows it
    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::Held, 200.0),
        unique_key(),
    )
    .await
    .expect("held in EarnedCredit");

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.displayed_balance - 200.0).abs() < f64::EPSILON,
        "held should be visible in displayed, got {}",
        balance.displayed_balance
    );
    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "held should NOT be spendable regardless of bucket, got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 13: Across movement with mismatched amounts
// The system does NOT validate amount parity between out and in entries
// ===========================================================================

#[tokio::test]
async fn test_across_movement_mismatched_amounts() {
    let (pool, _, wallet_id) = setup().await;

    // Seed some credit to make Out valid
    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 500.0),
        unique_key(),
    )
    .await
    .expect("seed credit");

    let out_entry = make_entry(wallet_id, BucketType::EarnedCredit, MovementType::Out, 100.0);
    let in_entry = make_entry(wallet_id, BucketType::GiftCard, MovementType::In, 200.0);

    // Amount mismatch: out 100, in 200 — system allows this
    let result = ledger_storage::create_across_movement(
        &pool,
        out_entry,
        unique_key(),
        in_entry,
        unique_key(),
    )
    .await;

    assert!(
        result.is_ok(),
        "across with mismatched amounts is accepted (no validation)"
    );

    let (out, r#in) = result.unwrap();
    assert!(
        (out.earning_unit - 100.0).abs() < f64::EPSILON,
        "out amount"
    );
    assert!(
        (r#in.earning_unit - 200.0).abs() < f64::EPSILON,
        "in amount (mismatched)"
    );
    assert_eq!(
        out.transfer_id, r#in.transfer_id,
        "transfer_id still links them"
    );
}

// ===========================================================================
// Edge Case 14: Across movement forces correct movement types
// Even if you pass In as the out_entry, it gets forced to Out
// ===========================================================================

#[tokio::test]
async fn test_across_movement_forces_movement_types() {
    let (pool, _, wallet_id) = setup().await;

    // Pass "In" for both — the system should force Out on the first
    let out_entry = make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 50.0);
    let in_entry = make_entry(wallet_id, BucketType::GiftCard, MovementType::Out, 50.0);

    let (out, r#in) = ledger_storage::create_across_movement(
        &pool,
        out_entry,
        unique_key(),
        in_entry,
        unique_key(),
    )
    .await
    .expect("across movement");

    assert_eq!(
        out.movement_type,
        MovementType::Out,
        "out entry must be forced to Out"
    );
    assert_eq!(
        r#in.movement_type,
        MovementType::In,
        "in entry must be forced to In"
    );
}

// ===========================================================================
// Edge Case 15: Idempotent across movement (replayed keys)
// ===========================================================================

#[tokio::test]
async fn test_across_movement_idempotent_on_replay() {
    let (pool, _, wallet_id) = setup().await;

    let out_key = unique_key();
    let in_key = unique_key();

    let out_entry = make_entry(wallet_id, BucketType::EarnedCredit, MovementType::Out, 50.0);
    let in_entry = make_entry(wallet_id, BucketType::GiftCard, MovementType::In, 50.0);

    let (out1, in1) = ledger_storage::create_across_movement(
        &pool,
        out_entry,
        out_key.clone(),
        in_entry,
        in_key.clone(),
    )
    .await
    .expect("first across");

    let out_entry2 = make_entry(wallet_id, BucketType::EarnedCredit, MovementType::Out, 50.0);
    let in_entry2 = make_entry(wallet_id, BucketType::GiftCard, MovementType::In, 50.0);

    let (out2, in2) = ledger_storage::create_across_movement(
        &pool,
        out_entry2,
        out_key,
        in_entry2,
        in_key,
    )
    .await
    .expect("replayed across (idempotent)");

    assert_eq!(out1.id, out2.id, "out entry must be same on replay");
    assert_eq!(in1.id, in2.id, "in entry must be same on replay");
}

// ===========================================================================
// Edge Case 16: Many small entries — balance accumulation
// ===========================================================================

#[tokio::test]
async fn test_many_small_entries_accumulate_correctly() {
    let (pool, _, wallet_id) = setup().await;

    let count = 100;
    for _ in 0..count {
        ledger_storage::create_entry(
            &pool,
            make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 1.0),
            unique_key(),
        )
        .await
        .expect("credit 1");
    }

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.spendable_balance - count as f64).abs() < 0.01,
        "100 x 1.0 should give 100.0, got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 17: Fractional amounts (paise-level precision)
// ===========================================================================

#[tokio::test]
async fn test_fractional_amount_precision() {
    let (pool, _, wallet_id) = setup().await;

    // 33.33 * 3 should be 99.99 exactly
    for _ in 0..3 {
        ledger_storage::create_entry(
            &pool,
            make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 33.33),
            unique_key(),
        )
        .await
        .expect("credit 33.33");
    }

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.spendable_balance - 99.99).abs() < 0.001,
        "3 x 33.33 should be 99.99, got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 18: Entries query pagination
// ===========================================================================

#[tokio::test]
async fn test_entries_pagination() {
    let (pool, _, wallet_id) = setup().await;

    for _ in 0..5 {
        ledger_storage::create_entry(
            &pool,
            make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 10.0),
            unique_key(),
        )
        .await
        .expect("credit");
    }

    let page1 = ledger_storage::get_entries(
        &pool,
        wallet_id,
        GetEntriesQuery {
            page: Some(1),
            limit: Some(2),
            bucket_type: None,
            movement_type: None,
        },
    )
    .await
    .expect("page 1");
    assert_eq!(page1.len(), 2, "page 1 should have 2 entries");

    let page2 = ledger_storage::get_entries(
        &pool,
        wallet_id,
        GetEntriesQuery {
            page: Some(2),
            limit: Some(2),
            bucket_type: None,
            movement_type: None,
        },
    )
    .await
    .expect("page 2");
    assert_eq!(page2.len(), 2, "page 2 should have 2 entries");

    let page3 = ledger_storage::get_entries(
        &pool,
        wallet_id,
        GetEntriesQuery {
            page: Some(3),
            limit: Some(2),
            bucket_type: None,
            movement_type: None,
        },
    )
    .await
    .expect("page 3");
    assert_eq!(page3.len(), 1, "page 3 should have 1 entry");

    // No overlap between pages
    assert_ne!(page1[0].id, page2[0].id, "pages must not overlap");
    assert_ne!(page2[0].id, page3[0].id, "pages must not overlap");
}

#[tokio::test]
async fn test_entries_filter_by_bucket_and_movement() {
    let (pool, _, wallet_id) = setup().await;

    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0),
        unique_key(),
    )
    .await
    .expect("earned in");

    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::GoodwillCredit, MovementType::In, 50.0),
        unique_key(),
    )
    .await
    .expect("goodwill in");

    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::Out, 20.0),
        unique_key(),
    )
    .await
    .expect("earned out");

    // Filter: EarnedCredit + In
    let filtered = ledger_storage::get_entries(
        &pool,
        wallet_id,
        GetEntriesQuery {
            page: Some(1),
            limit: Some(100),
            bucket_type: Some(BucketType::EarnedCredit),
            movement_type: Some(MovementType::In),
        },
    )
    .await
    .expect("filtered entries");

    assert_eq!(filtered.len(), 1, "should get 1 EarnedCredit In entry");
    assert_eq!(filtered[0].bucket_type, BucketType::EarnedCredit);
    assert_eq!(filtered[0].movement_type, MovementType::In);

    // Filter: movement_type only
    let outs = ledger_storage::get_entries(
        &pool,
        wallet_id,
        GetEntriesQuery {
            page: Some(1),
            limit: Some(100),
            bucket_type: None,
            movement_type: Some(MovementType::Out),
        },
    )
    .await
    .expect("out entries");

    assert_eq!(outs.len(), 1, "should get 1 Out entry");
}

// ===========================================================================
// Edge Case 19: COD double delivery rejected
// ===========================================================================

#[tokio::test]
async fn test_cod_double_delivery_rejected() {
    let (pool, merchant, wallet_id) = setup().await;

    let held_entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::CodPending, MovementType::Held, 100.0),
        unique_key(),
    )
    .await
    .expect("held");

    let order_id = format!("DBL-DEL-{}", Uuid::new_v4().as_simple());
    cod_storage::create_cod_order(&pool, merchant, &order_id, wallet_id, held_entry.id)
        .await
        .expect("create cod order");

    cod_helpers::process_delivery(&pool, &order_id, merchant)
        .await
        .expect("first delivery");

    let second = cod_helpers::process_delivery(&pool, &order_id, merchant).await;
    assert!(
        second.is_err(),
        "double delivery must be rejected (state is no longer pending)"
    );
}

// ===========================================================================
// Edge Case 20: COD double RTO rejected
// ===========================================================================

#[tokio::test]
async fn test_cod_double_rto_rejected() {
    let (pool, merchant, wallet_id) = setup().await;

    let held_entry = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::CodPending, MovementType::Held, 100.0),
        unique_key(),
    )
    .await
    .expect("held");

    let order_id = format!("DBL-RTO-{}", Uuid::new_v4().as_simple());
    cod_storage::create_cod_order(&pool, merchant, &order_id, wallet_id, held_entry.id)
        .await
        .expect("create cod order");

    cod_helpers::process_rto(&pool, &order_id, merchant)
        .await
        .expect("first rto");

    let second = cod_helpers::process_rto(&pool, &order_id, merchant).await;
    assert!(
        second.is_err(),
        "double RTO must be rejected (state is no longer pending)"
    );
}

// ===========================================================================
// Edge Case 21: RTO after delivery rejected (reverse direction)
// ===========================================================================

#[tokio::test]
async fn test_cod_rto_after_delivery_rejected() {
    let (pool, merchant, wallet_id) = setup().await;

    let held = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::CodPending, MovementType::Held, 90.0),
        unique_key(),
    )
    .await
    .expect("held");

    let order_id = format!("RTO-AFTER-DEL-{}", Uuid::new_v4().as_simple());
    cod_storage::create_cod_order(&pool, merchant, &order_id, wallet_id, held.id)
        .await
        .expect("cod order");

    cod_helpers::process_delivery(&pool, &order_id, merchant)
        .await
        .expect("delivery");

    let rto_result = cod_helpers::process_rto(&pool, &order_id, merchant).await;
    assert!(
        rto_result.is_err(),
        "RTO after delivery must fail"
    );

    // Balance should still reflect delivered credit
    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");
    assert!(
        (balance.spendable_balance - 90.0).abs() < f64::EPSILON,
        "spendable should be 90 (delivered credit), got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 22: Delivery after RTO rejected
// ===========================================================================

#[tokio::test]
async fn test_cod_delivery_after_rto_rejected() {
    let (pool, merchant, wallet_id) = setup().await;

    let held = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::CodPending, MovementType::Held, 70.0),
        unique_key(),
    )
    .await
    .expect("held");

    let order_id = format!("DEL-AFTER-RTO-{}", Uuid::new_v4().as_simple());
    cod_storage::create_cod_order(&pool, merchant, &order_id, wallet_id, held.id)
        .await
        .expect("cod order");

    cod_helpers::process_rto(&pool, &order_id, merchant)
        .await
        .expect("rto");

    let delivery_result = cod_helpers::process_delivery(&pool, &order_id, merchant).await;
    assert!(
        delivery_result.is_err(),
        "delivery after RTO must fail"
    );

    // Balance should be 0 (RTO cancelled the held credit)
    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");
    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "spendable should be 0 after RTO, got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 23: Historical balance with mixed expiry
// ===========================================================================

#[tokio::test]
async fn test_historical_balance_respects_expiry_at_query_time() {
    let (pool, _, wallet_id) = setup().await;

    let now = Utc::now();

    // Entry 1: expires in 1 hour
    let mut entry1 = make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0);
    entry1.expires_at = Some(now + Duration::hours(1));
    ledger_storage::create_entry(&pool, entry1, unique_key())
        .await
        .expect("entry with 1h expiry");

    // Entry 2: no expiry
    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 200.0),
        unique_key(),
    )
    .await
    .expect("entry with no expiry");

    // Query at now + 30min: both visible
    let at_30min = ledger_storage::get_balance_at(&pool, wallet_id, now + Duration::minutes(30))
        .await
        .expect("balance at +30min");
    assert!(
        (at_30min.spendable_balance - 300.0).abs() < 1.0,
        "both entries visible at +30min, got {}",
        at_30min.spendable_balance
    );

    // Query at now + 2 hours: only entry 2 visible
    let at_2h = ledger_storage::get_balance_at(&pool, wallet_id, now + Duration::hours(2))
        .await
        .expect("balance at +2h");
    assert!(
        (at_2h.spendable_balance - 200.0).abs() < 1.0,
        "only non-expiring entry visible at +2h, got {}",
        at_2h.spendable_balance
    );
}

// ===========================================================================
// Edge Case 24: Wallet isolation — entries don't leak between wallets
// ===========================================================================

#[tokio::test]
async fn test_wallet_isolation_no_cross_contamination() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    let customer_a = create_test_customer(&pool).await;
    let customer_b = create_test_customer(&pool).await;
    let wallet_a = create_test_wallet(&pool, merchant, customer_a).await;
    let wallet_b = create_test_wallet(&pool, merchant, customer_b).await;

    ledger_storage::create_entry(
        &pool,
        make_entry(wallet_a, BucketType::EarnedCredit, MovementType::In, 1000.0),
        unique_key(),
    )
    .await
    .expect("credit wallet A");

    let balance_b = ledger_storage::get_balance(&pool, wallet_b)
        .await
        .expect("balance B");
    assert!(
        (balance_b.spendable_balance - 0.0).abs() < f64::EPSILON,
        "wallet B must not see wallet A's balance, got {}",
        balance_b.spendable_balance
    );
}

// ===========================================================================
// Edge Case 25: Idempotency key collision across different wallets
// Same idempotency key used for different wallets returns the FIRST entry
// ===========================================================================

#[tokio::test]
async fn test_idempotency_key_is_global_not_wallet_scoped() {
    let pool = get_test_pool().await;
    let merchant = create_test_merchant(&pool).await;

    let customer_a = create_test_customer(&pool).await;
    let customer_b = create_test_customer(&pool).await;
    let wallet_a = create_test_wallet(&pool, merchant, customer_a).await;
    let wallet_b = create_test_wallet(&pool, merchant, customer_b).await;

    let shared_key = format!("shared-{}", Uuid::new_v4());

    let entry_a = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_a, BucketType::EarnedCredit, MovementType::In, 100.0),
        shared_key.clone(),
    )
    .await
    .expect("first entry with shared key");

    // Same key, different wallet — should return the FIRST entry (wallet A's)
    let entry_b = ledger_storage::create_entry(
        &pool,
        make_entry(wallet_b, BucketType::EarnedCredit, MovementType::In, 200.0),
        shared_key,
    )
    .await
    .expect("second call returns existing");

    assert_eq!(
        entry_a.id, entry_b.id,
        "same idempotency key must return same entry regardless of wallet"
    );
    assert_eq!(
        entry_b.wallet_id, wallet_a,
        "returned entry belongs to wallet A, not B — key is global"
    );

    // Wallet B should have 0 balance — nothing was created for it
    let balance_b = ledger_storage::get_balance(&pool, wallet_b)
        .await
        .expect("balance B");
    assert!(
        (balance_b.spendable_balance - 0.0).abs() < f64::EPSILON,
        "wallet B must have 0 balance since idempotency returned wallet A's entry"
    );
}

// ===========================================================================
// Edge Case 26: Multiple Out entries without any In
// ===========================================================================

#[tokio::test]
async fn test_multiple_outs_without_ins_clamp_to_zero() {
    let (pool, _, wallet_id) = setup().await;

    for _ in 0..3 {
        ledger_storage::create_entry(
            &pool,
            make_entry(wallet_id, BucketType::EarnedCredit, MovementType::Out, 100.0),
            unique_key(),
        )
        .await
        .expect("out without prior in");
    }

    let balance = ledger_storage::get_balance(&pool, wallet_id)
        .await
        .expect("balance");

    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "3 outs with no ins should clamp to 0, got {}",
        balance.spendable_balance
    );
}

// ===========================================================================
// Edge Case 27: Entry with non-wallet UUID — foreign key violation
// ===========================================================================

#[tokio::test]
async fn test_entry_for_nonexistent_wallet_fails() {
    let pool = get_test_pool().await;
    let fake_wallet_id = Uuid::new_v4();

    let result = ledger_storage::create_entry(
        &pool,
        make_entry(fake_wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0),
        unique_key(),
    )
    .await;

    assert!(
        result.is_err(),
        "entry for nonexistent wallet must fail with FK violation"
    );
}

// ===========================================================================
// Edge Case 28: Concurrent idempotency — first writer wins
// ===========================================================================

#[tokio::test]
async fn test_concurrent_same_idempotency_key() {
    let (pool, _, wallet_id) = setup().await;

    let shared_key = format!("concurrent-{}", Uuid::new_v4());

    // Simulate two concurrent inserts — both should return the same entry
    let pool_clone = pool.clone();
    let key_clone = shared_key.clone();
    let handle1 = tokio::spawn(async move {
        ledger_storage::create_entry(
            &pool_clone,
            make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 100.0),
            key_clone,
        )
        .await
    });

    let pool_clone2 = pool.clone();
    let key_clone2 = shared_key.clone();
    let handle2 = tokio::spawn(async move {
        ledger_storage::create_entry(
            &pool_clone2,
            make_entry(wallet_id, BucketType::EarnedCredit, MovementType::In, 200.0),
            key_clone2,
        )
        .await
    });

    let result1 = handle1.await.expect("task 1");
    let result2 = handle2.await.expect("task 2");

    // At least one should succeed
    assert!(
        result1.is_ok() || result2.is_ok(),
        "at least one concurrent insert must succeed"
    );

    // If both succeed, they must return the same entry
    if let (Ok(e1), Ok(e2)) = (&result1, &result2) {
        assert_eq!(
            e1.id, e2.id,
            "concurrent inserts with same key must return same entry"
        );
    }

    // Balance should only count ONE entry, not two
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
    .expect("entries");

    let matching: Vec<_> = entries
        .iter()
        .filter(|e| e.idempotency_key == shared_key)
        .collect();
    assert_eq!(
        matching.len(),
        1,
        "only one entry must exist for the shared idempotency key"
    );
}

// ===========================================================================
// Edge Case 29: Balance query on nonexistent wallet returns zero
// ===========================================================================

#[tokio::test]
async fn test_balance_for_nonexistent_wallet_returns_zero() {
    let pool = get_test_pool().await;
    let fake_wallet = Uuid::new_v4();

    let balance = ledger_storage::get_balance(&pool, fake_wallet)
        .await
        .expect("balance for nonexistent wallet should not error");

    assert!(
        (balance.spendable_balance - 0.0).abs() < f64::EPSILON,
        "nonexistent wallet balance must be 0"
    );
    assert!(balance.buckets.is_empty(), "no buckets for nonexistent wallet");
}

// ===========================================================================
// Edge Case 30: Entries with all optional fields as None
// ===========================================================================

#[tokio::test]
async fn test_entry_with_all_optional_fields_none() {
    let (pool, _, wallet_id) = setup().await;

    let entry = NewLedgerEntry {
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
        payment_reference: None,
        transfer_id: None,
        constraints: json!({}),
        expires_at: None,
    };

    let result = ledger_storage::create_entry(&pool, entry, unique_key())
        .await
        .expect("entry with all optional fields None");

    assert!(result.event_id.is_none());
    assert!(result.rule_snapshot_id.is_none());
    assert!(result.campaign_snapshot_id.is_none());
    assert!(result.actor_id.is_none());
    assert!(result.payment_reference.is_none());
    assert!(result.transfer_id.is_none());
    assert!(result.expires_at.is_none());
}
