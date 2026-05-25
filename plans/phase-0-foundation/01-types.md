# Phase 0: Types -- COMPLETED

All types defined during the foundation phase, grouped by service.

## Shared Error Type

`src/error.rs` -- `AppError` enum with variants:
- `Database(sqlx::Error)` -- 500
- `Redis(redis::RedisError)` -- 500
- `NotFound(String)` -- 404
- `BadRequest(String)` -- 400
- `Internal(String)` -- 500
- `Conflict(String)` -- 409
- `Unauthorized(String)` -- 401

## AppState

`src/app_state.rs`:
- `AppState { db: PgPool, db_reader: Option<PgPool>, redis: redis::Client }`

## Ledger Types

`src/services/ledger/types.rs`:

### Enums (sqlx-mapped to Postgres custom types)
- `MovementType` -- In, Held, Out, Across
- `ActorType` -- System, Human, Automation, Migration
- `BucketType` -- EarnedCredit, CodPending, GiftCard, CustomerFunded, ReferralReward, GoodwillCredit, MembershipBenefit, RefundCredit
- `CreditState` -- Active, Expired, Redeemed, Reversed, Cancelled

### Structs
- `LedgerEntry` (sqlx::FromRow) -- full row with wallet_id, bucket_type, movement_type, earning_unit/currency_equivalent/conversion_rate triple, idempotency_key, cause chain (event_id, rule_snapshot_id, campaign_snapshot_id), actor, constraints JSONB, expires_at, state
- `LedgerEntryDetail` (sqlx::FromRow) -- enriched view joining customer name/phone/email, rule name, campaign name, event type, linked entry
- `NewLedgerEntry` -- insert payload (no id, no created_at)
- `WalletBalance` -- wallet_id, displayed_balance, spendable_balance, Vec<BucketBalance>
- `BucketBalance` -- per-bucket displayed/spendable/count
- `CreateEntryRequest` -- handler-level request body
- `GetEntriesQuery` -- pagination + optional bucket_type/movement_type filters
- `BalanceAtQuery` -- point-in-time balance query

## Wallet Types

`src/services/wallets/types.rs`:
- `Wallet` (sqlx::FromRow) -- id, merchant_id, customer_id (optional for bearer), is_bearer, bearer_code, created_at
- `CreateWalletRequest` -- merchant_id, customer_id, is_bearer, bearer_code
- `WalletResponse` -- wallet + optional balance summary
- `WalletBalanceSummary` -- displayed_balance, spendable_balance
- `WalletLookupQuery` -- merchant_id + customer_id
- `GetOrCreateRequest` -- merchant_id + customer_id
- `PaginationQuery` -- page, limit

## Identity Types

`src/services/identity/types.rs`:
- `Customer` (sqlx::FromRow) -- id, phone, email, name, external_id, is_verified, birthday (NaiveDate), timestamps
- `IdentityResolution` -- customer_id, is_verified, is_new
- `ResolveIdentityRequest` -- phone (required), email, name, external_id
- `UpdateCustomerRequest` -- all optional: email, name, external_id, is_verified, birthday
- `CustomerQuery` -- phone, external_id (at least one required)

`src/services/identity/storage.rs`:
- `CustomerWithWallet` (sqlx::FromRow) -- customer fields + wallet_id, used for birthday bonus queries

## Events Types

`src/services/events/types.rs`:

### Enums
- `EventState` -- Received, Processing, Processed, Failed, Duplicate

### Structs
- `Event` (sqlx::FromRow) -- id, merchant_id, event_type, event_source, external_event_id, payload JSONB, state, idempotency_key, timestamps
- `IngestEventRequest` -- merchant_id, event_type, event_source, external_event_id, payload
- `ShopifyOrderPayload` -- id, order_number, email, phone, total_price, currency, financial_status, gateway, payment_gateway_names, customer, line_items
- `ShopifyCustomer` -- id, email, phone, first_name, last_name
- `EventResponse` -- event_id, state, is_duplicate
- `ListEventsQuery` -- optional merchant_id, event_type, state, limit, offset
- `ShopifyWebhookRequest` -- merchant_id + raw payload

## Earn Types

`src/services/earn/types.rs`:
- `ProcessEarnRequest` -- event_id
- `EarnResult` -- event_id, customer_id, wallet_id, entries_created, is_cod
- `EarnEntry` -- ledger_entry_id, bucket_type, earning_unit, currency_equivalent, movement_type
- `ManualCreditRequest` -- merchant_id, customer_id, amount, bucket_type, reason, actor_id
- `ManualCreditResult` -- ledger_entry_id, wallet_id, amount
- `ProcessBirthdayBonusRequest` / `BirthdayBonusResult` / `BirthdayBonusEntry`
- `MilestoneConfig` (sqlx::FromRow), `CreateMilestoneRequest`, `CheckMilestonesRequest`, `MilestoneCheckResult`, `MilestoneAchievementEntry`, `AchievedMilestone` (sqlx::FromRow)
- `NewsletterSignupRequest` / `NewsletterSignupResult` / `NewsletterSignupCount`
- `ProfileCompletionRequest` / `ProfileCompletionResult`
- `StreakConfig` (sqlx::FromRow), `CreateStreakConfigRequest`, `CheckStreakRequest`, `StreakCheckResult`, `StreakAchievementEntry`, `ActiveStreak`
- `SpinWheelConfig` (sqlx::FromRow), `SpinWheelSegment` (sqlx::FromRow), `CreateWheelRequest`, `CreateSegmentRequest`, `SpinRequest`, `SpinResult`, `WheelWithSegments`
- `MembershipPlan` (sqlx::FromRow), `CustomerMembership` (sqlx::FromRow), `CreateMembershipPlanRequest`, `SubscribeRequest`, `SubscribeResult`, `RenewRequest`, `MembershipStatus`, `CancelMembershipRequest`

`src/services/earn/storage.rs`:
- `CustomerOrderStats` (sqlx::FromRow) -- total_orders, total_spend, first_order_at, last_order_at

## Redemption Types

`src/services/redemption/types.rs`:

### Enums
- `RedemptionState` -- Initiated, Validating, Rejected, Committed, Applied, Failed, Compensated, Completed

### Structs
- `RedemptionRequest` (sqlx::FromRow) -- full redemption row with state machine, linked entry IDs, shopify discount ID
- `WalletPolicy` (sqlx::FromRow) -- per-bucket redemption/earn constraints (min, step, max pct/fixed, stackability, transferability, excluded payment methods/collections)
- `InitiateRedemptionRequest` -- wallet_id, order context, requested_amount, discount_codes
- `BucketEligibility`, `RedemptionEligibility` -- eligibility evaluation results
- `BucketDebit`, `RedemptionResponse` -- debit execution results
- `EligibilityQuery` -- order_amount, payment_method
- `OrderContext` -- order_id, order_amount, payment_method, discount_codes

## COD Types

`src/services/cod/types.rs`:
- `CodOrderState` -- Pending, Delivered, Rto, Cancelled (with as_str/from_str)
- `CodOrder` (sqlx::FromRow) -- merchant_id, order_id, wallet_id, ledger_entry_id, state, delivery timestamps, released/cancelled entry IDs
- `DeliveryWebhookPayload` -- order_id, status, delivered_at, merchant_id
- `CodToPrepaidRequest` / `CodToPrepaidResponse`
- `CodOrderResponse`, `CodAnalytics`, `CodOrdersQuery`
