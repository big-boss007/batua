# Phase 1: Types -- COMPLETED

## Ledger Types (`src/services/ledger/types.rs`)

### Enums

```
MovementType    -- In | Held | Out | Across
ActorType       -- System | Human | Automation | Migration
BucketType      -- EarnedCredit | CodPending | GiftCard | CustomerFunded | ReferralReward | GoodwillCredit | MembershipBenefit | RefundCredit
CreditState     -- Active | Expired | Redeemed | Reversed | Cancelled
```

All enums derive `sqlx::Type` with `rename_all` matching the Postgres enum values.

### Core Structs

- `LedgerEntry` -- Full ledger row (`sqlx::FromRow`). Fields: id, wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, idempotency_key, event_id, rule_snapshot_id, campaign_snapshot_id, actor_type, actor_id, payment_reference, transfer_id, constraints (JSONB), expires_at, created_at, state.
- `NewLedgerEntry` -- Insert DTO (no id, no idempotency_key, no created_at, no state).
- `WalletBalance` -- Aggregated balance: wallet_id, displayed_balance, spendable_balance, buckets (Vec<BucketBalance>).
- `BucketBalance` -- Per-bucket: bucket_type, displayed, spendable, count.
- `LedgerEntryDetail` -- Enriched entry with joined customer/rule/campaign names for admin views.

## Wallet Types (`src/services/wallets/types.rs`)

- `Wallet` -- id, merchant_id, customer_id (Option), is_bearer, bearer_code (Option), created_at.
- `CreateWalletRequest` -- merchant_id, customer_id (Option), is_bearer, bearer_code (Option).
- `WalletResponse` -- wallet + optional WalletBalanceSummary.
- `WalletBalanceSummary` -- displayed_balance, spendable_balance.
- `WalletLookupQuery` -- merchant_id + customer_id.
- `GetOrCreateRequest` -- merchant_id + customer_id.
- `PaginationQuery` -- page (Option), limit (Option).

## Earn Types (`src/services/earn/types.rs`)

### Order Earn

- `ProcessEarnRequest` -- event_id.
- `EarnResult` -- event_id, customer_id, wallet_id, entries_created (Vec<EarnEntry>), is_cod.
- `EarnEntry` -- ledger_entry_id, bucket_type, earning_unit, currency_equivalent, movement_type.
- `ManualCreditRequest` -- merchant_id, customer_id, amount, bucket_type, reason, actor_id.
- `ManualCreditResult` -- ledger_entry_id, wallet_id, amount.

### Birthday Bonus

- `ProcessBirthdayBonusRequest` -- merchant_id, amount.
- `BirthdayBonusResult` -- merchant_id, processed, credited, skipped, entries (Vec<BirthdayBonusEntry>).
- `BirthdayBonusEntry` -- customer_id, customer_name (Option), amount, ledger_entry_id.

### Milestones

- `MilestoneConfig` (sqlx::FromRow) -- id, merchant_id, name, milestone_type, threshold, reward_amount, is_active, created_at.
- `AchievedMilestone` (sqlx::FromRow) -- Same fields + achieved_at.
- `CreateMilestoneRequest` -- merchant_id, name, milestone_type, threshold, reward_amount.
- `CheckMilestonesRequest` -- merchant_id, customer_id.
- `MilestoneCheckResult` -- customer_id, milestones_achieved (Vec<MilestoneAchievementEntry>).
- `MilestoneAchievementEntry` -- milestone_name, reward_amount, ledger_entry_id.

### Newsletter Signup

- `NewsletterSignupRequest` -- merchant_id, email, phone (Option), customer_id (Option), amount.
- `NewsletterSignupResult` -- customer_id, email, rewarded, already_subscribed, ledger_entry_id (Option), amount.
- `NewsletterSignupCount` -- merchant_id, count.

### Profile Completion

- `ProfileCompletionRequest` -- merchant_id, customer_id.
- `ProfileCompletionResult` -- customer_id, fields_complete, fields_missing, completion_pct, already_rewarded, rewarded, amount, ledger_entry_id (Option).

### Streaks

- `StreakConfig` (sqlx::FromRow) -- id, merchant_id, name, required_orders, window_days, reward_amount, is_active, created_at.
- `CreateStreakConfigRequest` -- merchant_id, name, required_orders, window_days, reward_amount.
- `CheckStreakRequest` -- merchant_id, customer_id.
- `StreakCheckResult` -- customer_id, streaks_achieved (Vec<StreakAchievementEntry>), active_streaks (Vec<ActiveStreak>).
- `ActiveStreak` -- streak_name, required_orders, orders_in_window, window_days, progress_pct.

### Spin Wheel

- `SpinWheelConfig` (sqlx::FromRow) -- id, merchant_id, name, is_active, daily_spin_limit, created_at.
- `SpinWheelSegment` (sqlx::FromRow) -- id, wheel_id, label, reward_amount, probability, color, position, created_at.
- `CreateWheelRequest` -- merchant_id, name (Option), daily_spin_limit (Option), segments (Vec<CreateSegmentRequest>).
- `SpinRequest` -- merchant_id, customer_id.
- `SpinResult` -- segment, reward_amount, ledger_entry_id (Option), spins_remaining_today.
- `WheelWithSegments` -- config + segments.

### Memberships

- `MembershipPlan` (sqlx::FromRow) -- id, merchant_id, name, plan_type, price, earn_rate_multiplier, benefits (JSON), is_active, created_at.
- `CustomerMembership` (sqlx::FromRow) -- id, merchant_id, customer_id, plan_id, status, started_at, expires_at, renewed_count, cancelled_at (Option), created_at.
- `CreateMembershipPlanRequest` -- merchant_id, name, plan_type, price, earn_rate_multiplier (Option), benefits (Option).
- `SubscribeRequest` -- merchant_id, customer_id, plan_id.
- `SubscribeResult` -- membership, plan, is_new, message.
- `RenewRequest` -- membership_id.
- `MembershipStatus` -- membership (Option), plan (Option), is_active, days_remaining.
- `CancelMembershipRequest` -- membership_id.

## Redemption Types (`src/services/redemption/types.rs`)

### Enums

```
RedemptionState -- Initiated | Validating | Rejected | Committed | Applied | Failed | Compensated | Completed
```

### Structs

- `RedemptionRequest` (sqlx::FromRow) -- id, merchant_id, wallet_id, requested_amount, eligible_amount (Option), applied_amount (Option), order_id, order_amount, payment_method (Option), state, debit_entry_id (Option), compensation_entry_id (Option), shopify_discount_id (Option), rejection_reason (Option), created_at, updated_at.
- `WalletPolicy` (sqlx::FromRow) -- id, merchant_id, bucket_type, min_redemption, step_size, max_per_order_pct, max_per_order_fixed, stackable_with_discounts, default_conversion_rate, default_expiry_days, is_transferable, excluded_payment_methods (Vec<String>), excluded_collections (Vec<String>), is_active, created_at, updated_at.
- `InitiateRedemptionRequest` -- wallet_id, order_id, order_amount, payment_method (Option), requested_amount, discount_codes (Vec<String>).
- `BucketEligibility` -- bucket_type, eligible_amount, constraints (JSON).
- `RedemptionEligibility` -- total_eligible, buckets (Vec<BucketEligibility>).
- `BucketDebit` -- bucket_type, amount, entry_id.
- `RedemptionResponse` -- redemption_id, state, applied_amount (Option), buckets_debited (Vec<BucketDebit>).
- `EligibilityQuery` -- order_amount, payment_method (Option).
- `OrderContext` -- order_id, order_amount, payment_method (Option), discount_codes (Vec<String>).

## COD Types (`src/services/cod/types.rs`)

### Enums

```
CodOrderState -- Pending | Delivered | Rto | Cancelled
```

Manual `as_str()` / `from_str()` implementations (not sqlx enum -- stored as TEXT).

### Structs

- `CodOrder` (sqlx::FromRow) -- id, merchant_id, order_id, wallet_id, ledger_entry_id, state (String), delivery_confirmed_at (Option), released_entry_id (Option), cancelled_entry_id (Option), created_at, updated_at.
- `DeliveryWebhookPayload` -- order_id, status, delivered_at (Option), merchant_id (Option).
- `CodToPrepaidRequest` -- merchant_id, order_id, customer_id, order_amount, new_payment_method.
- `CodToPrepaidResponse` -- incentive_amount, ledger_entry_id, message.
- `CodOrderResponse` -- id, order_id, state, pending_amount, created_at.
- `CodAnalytics` -- total_pending, total_delivered, total_rto, pending_amount, released_amount, cancelled_amount.
- `CodOrdersQuery` -- state (Option), page (Option), limit (Option).
