# Phase 1: Helpers -- COMPLETED

## Wallets (`src/services/wallets/helpers.rs`)

- `generate_bearer_code() -> String` -- Generates a 16-character alphanumeric code using UUID bytes as entropy source. Used for gift card bearer tokens before claim.

## Earn (`src/services/earn/helpers.rs`)

### Order Cashback Flow

`process_earn(pool, event_id) -> EarnResult`

1. Fetch event, verify state is `Received`.
2. Mark event `Processing`.
3. Parse Shopify order payload; extract customer phone/email/name.
4. Parse `total_price` from order.
5. Detect COD via `events::helpers::is_cod_order`.
6. Resolve or create customer identity via `identity::storage::resolve_or_create`.
7. Get or create wallet for (merchant, customer).
8. Check if first order (no prior order stats).
9. Build `EvaluationContext` and evaluate rules via `rules::helpers::evaluate_rules`.
10. For each matched rule with positive earning_unit:
    - COD orders: movement = `Held`, bucket = `CodPending`.
    - Non-COD: movement = `In`, bucket from rule evaluation.
    - Generate deterministic idempotency key: `earn:{event_id}:{rule_snapshot_id}`.
    - Calculate expiry from rule's `expiry_days`.
    - Create ledger entry.
    - If COD: also create cod_order record.
11. Update customer order stats.
12. On success: mark event `Processed`. On failure: mark event `Failed`.

### Manual Credit

`process_manual_credit(pool, req) -> ManualCreditResult`

- Get or create wallet, create ledger entry with `ActorType::Human`, `MovementType::In`.
- Uses UUID-based idempotency key (non-deterministic -- each call creates a new entry).

### Birthday Bonus

`process_birthday_bonuses(pool, merchant_id, amount) -> BirthdayBonusResult`

- Fetches customers with birthday today via identity service.
- For each: SHA-256 hash of `{merchant_id}{customer_id}birthday{date}` for idempotency.
- Credits `EarnedCredit` bucket with specified amount.
- Returns processed/credited/skipped counts.

### Newsletter Signup

`process_newsletter_signup(pool, req) -> NewsletterSignupResult`

- Validates email format.
- Resolves customer: by customer_id, phone, or email.
- Checks for prior signup (dedup).
- Credits `EarnedCredit` bucket.
- Records signup in `newsletter_signups` table.

### Profile Completion

`process_profile_completion(pool, req) -> ProfileCompletionResult`

- Checks name, email, birthday fields on customer.
- Returns completion percentage and missing fields.
- If 100% complete and not previously rewarded: credits 30.0 `EarnedCredit`.
- Idempotency via SHA-256 of `{merchant_id}{customer_id}profile_complete`.

### Milestones

`check_and_award_milestones(pool, merchant_id, customer_id) -> MilestoneCheckResult`

- Fetches active milestone configs for merchant.
- Compares customer order stats against thresholds.
- Milestone types: `order_count`, `lifetime_spend`.
- Awards `EarnedCredit` for each newly crossed threshold.
- Records achievement (ON CONFLICT DO NOTHING for safety).

### Streaks

`check_and_award_streaks(pool, merchant_id, customer_id) -> StreakCheckResult`

- Fetches active streak configs for merchant.
- Counts recent orders within each config's window_days.
- Returns progress percentage for each active streak.
- Awards `EarnedCredit` when required_orders threshold is met.
- Prevents double-award in same window via `has_streak_achievement_in_window`.

### Spin Wheel

`create_wheel(pool, req) -> WheelWithSegments`

- Creates wheel config (one per merchant) with segments.
- Default name "Lucky Wheel", default daily limit 1.

`spin_wheel(pool, req) -> SpinResult`

- Validates wheel is active and spins remaining today.
- Weighted random selection using UUID entropy.
- Credits `EarnedCredit` if winning segment has reward_amount > 0.
- Records spin result.

### Memberships

`subscribe_to_plan(pool, req) -> SubscribeResult`

- Validates plan is active. Checks for existing subscription.
- Calculates expiry: 365 days for "annual", 30 days otherwise.
- Returns is_new flag.

`renew_membership(pool, req) -> SubscribeResult`

- Extends from current expiry (or now if expired).
- Increments renewed_count.

`get_membership_status(pool, merchant_id, customer_id) -> MembershipStatus`

- Auto-expires active memberships past expiry date.
- Returns is_active flag and days_remaining.

`cancel_membership_by_id(pool, membership_id) -> CustomerMembership`

- Sets status to 'cancelled' with cancelled_at timestamp.

### Shared Utilities

- `parse_bucket_type(s) -> BucketType` -- Accepts both snake_case and PascalCase.
- `generate_earn_idempotency_key(event_id, rule_snapshot_id) -> String` -- Deterministic key.
- `parse_order_payload(payload) -> ShopifyOrderPayload` -- Deserialize from JSON.
- `extract_customer_phone(payload) -> String` -- From customer.phone or order.phone.
- `extract_customer_email(payload) -> Option<String>` -- From customer.email or order.email.
- `extract_customer_name(payload) -> Option<String>` -- Concatenates first_name + last_name.
- `build_evaluation_context(event, ...) -> EvaluationContext` -- Constructs rule evaluation input.

## Redemption (`src/services/redemption/helpers.rs`)

### Eligibility

`evaluate_eligibility(pool, wallet_id, order_context) -> RedemptionEligibility`

- Fetches wallet balance and merchant policies.
- For each bucket with positive spendable balance:
  - Checks if policy is active.
  - Excludes buckets if payment method is in `excluded_payment_methods`.
  - Applies `max_per_order_pct` and `max_per_order_fixed` caps.
  - Builds constraint summary JSON.

`validate_constraints(eligibility, requested_amount, policies, discount_codes) -> f64`

- Rejects if amount exceeds total eligible.
- Rejects if discount codes present and not all policies are `stackable_with_discounts`.
- Enforces `min_redemption` per bucket policy.
- Rounds down to `step_size` if configured.

### Execution

`execute_redemption(pool, redemption_id) -> RedemptionResponse`

State machine progression:
1. `Initiated` -> `Validating`: evaluate eligibility.
2. `Validating` -> `Rejected`: if constraints fail (records rejection_reason).
3. `Validating` -> `Committed`: if constraints pass (records eligible_amount, applied_amount).
4. `Committed` -> `Applied`: create debit entries (MovementType::Out) per bucket.
5. `Applied` -> `Completed`: apply Shopify discount (currently stub).
6. `Applied` -> `Failed`: if Shopify call fails, triggers compensation.

`create_debit_entries(pool, redemption, eligibility, total_amount, redemption_id) -> Vec<BucketDebit>`

- Iterates eligible buckets, debits up to eligible_amount per bucket.
- Idempotency key: `redemption-{redemption_id}-{bucket_type}`.
- Constraints JSON includes redemption_id and order_id.

### Compensation

`compensate(pool, redemption_id) -> ()`

- Only allowed from states: Failed, Applied, Committed.
- Credits back the applied_amount into the first available bucket.
- Compensation entries have `requires_review: true` in constraints.
- Updates state to `Compensated` with compensation_entry_id.

## COD (`src/services/cod/helpers.rs`)

### Delivery (Hold -> Release)

`process_delivery(pool, order_id, merchant_id) -> ()`

1. Fetch COD order, verify state is `pending`.
2. Find the held ledger entry for this order.
3. Create an `Across` movement: Out from `CodPending` + In to `EarnedCredit` via `ledger_storage::create_across_movement`.
4. Update COD order state to `delivered` with released_entry_id and delivery_confirmed_at.

### RTO (Hold -> Cancel)

`process_rto(pool, order_id, merchant_id) -> ()`

1. Fetch COD order, verify state is `pending`.
2. Find the held ledger entry.
3. Create a single `Out` from `CodPending` (value is cancelled, not transferred).
4. Update COD order state to `rto` with cancelled_entry_id.

### COD-to-Prepaid Incentive

`process_cod_to_prepaid(pool, req) -> CodToPrepaidResponse`

- Only supports `upi` as new_payment_method.
- Calculates incentive: max(2% of order_amount, 75.0), capped at order_amount.
- Credits `EarnedCredit` to customer's wallet.

`calculate_cod_incentive(order_amount) -> f64` -- Pure function for the incentive formula.
