# Phase 1: Storage -- COMPLETED

## Wallets (`src/services/wallets/storage.rs`)

| Function | Signature | Notes |
|----------|-----------|-------|
| `create_wallet` | `(pool, req) -> Wallet` | INSERT with unique violation -> AppError::Conflict |
| `get_wallet` | `(pool, id) -> Wallet` | Fetch by UUID, 404 if missing |
| `get_wallet_by_merchant_customer` | `(pool, merchant_id, customer_id) -> Option<Wallet>` | Lookup by composite key |
| `get_or_create_wallet` | `(pool, merchant_id, customer_id) -> Wallet` | INSERT ON CONFLICT DO NOTHING + fallback SELECT |
| `get_bearer_wallet` | `(pool, bearer_code) -> Option<Wallet>` | Lookup by bearer_code WHERE is_bearer = true |
| `list_wallets_for_merchant` | `(pool, merchant_id, page, limit) -> Vec<Wallet>` | Paginated, ordered by created_at DESC |

## Earn (`src/services/earn/storage.rs`)

### Order Stats

| Function | Signature | Notes |
|----------|-----------|-------|
| `get_customer_order_stats` | `(pool, merchant_id, customer_id) -> Option<CustomerOrderStats>` | From `customer_order_stats` table |
| `update_order_stats` | `(pool, merchant_id, customer_id, order_amount) -> ()` | Upsert: increment total_orders, add to total_spend |

`CustomerOrderStats` is a private `sqlx::FromRow` struct: `total_orders`, `total_spend`, `first_order_at`, `last_order_at`.

### Milestones

| Function | Signature | Notes |
|----------|-----------|-------|
| `create_milestone_config` | `(pool, req) -> MilestoneConfig` | INSERT RETURNING * |
| `get_active_milestones` | `(pool, merchant_id) -> Vec<MilestoneConfig>` | WHERE is_active = true, ORDER BY threshold |
| `has_achieved_milestone` | `(pool, customer_id, milestone_id) -> bool` | Existence check in milestone_achievements |
| `record_milestone_achievement` | `(pool, merchant_id, customer_id, milestone_id, ledger_entry_id) -> ()` | INSERT ON CONFLICT DO NOTHING |
| `get_customer_milestones` | `(pool, merchant_id, customer_id) -> Vec<AchievedMilestone>` | JOIN milestone_configs + milestone_achievements |

### Newsletter

| Function | Signature | Notes |
|----------|-----------|-------|
| `has_newsletter_signup` | `(pool, merchant_id, customer_id) -> bool` | Existence check |
| `record_newsletter_signup` | `(pool, merchant_id, customer_id, ledger_entry_id, email, source) -> ()` | INSERT ON CONFLICT DO NOTHING |
| `get_newsletter_signup_count` | `(pool, merchant_id) -> NewsletterSignupCount` | COUNT(*) |

### Streaks

| Function | Signature | Notes |
|----------|-----------|-------|
| `create_streak_config` | `(pool, req) -> StreakConfig` | INSERT RETURNING * |
| `get_active_streak_configs` | `(pool, merchant_id) -> Vec<StreakConfig>` | WHERE is_active = true |
| `count_recent_orders` | `(pool, merchant_id, customer_id, days) -> i64` | COUNT DISTINCT payment_reference from ledger_entries within window |
| `has_streak_achievement_in_window` | `(pool, customer_id, streak_config_id, window_start) -> bool` | Prevents double-award in same window |
| `record_streak_achievement` | `(pool, merchant_id, customer_id, streak_config_id, ledger_entry_id, window_start, window_end) -> ()` | INSERT |

### Spin Wheel

| Function | Signature | Notes |
|----------|-----------|-------|
| `create_wheel_config` | `(pool, merchant_id, name, daily_spin_limit) -> SpinWheelConfig` | INSERT RETURNING * |
| `create_wheel_segment` | `(pool, wheel_id, label, reward_amount, probability, color, position) -> SpinWheelSegment` | INSERT RETURNING * |
| `get_wheel_config` | `(pool, merchant_id) -> Option<SpinWheelConfig>` | One wheel per merchant |
| `get_wheel_segments` | `(pool, wheel_id) -> Vec<SpinWheelSegment>` | ORDER BY position |
| `count_spins_today` | `(pool, merchant_id, customer_id) -> i64` | WHERE spun_at >= CURRENT_DATE |
| `record_spin_result` | `(pool, merchant_id, customer_id, segment_id, reward_amount, ledger_entry_id) -> ()` | INSERT |

### Memberships

| Function | Signature | Notes |
|----------|-----------|-------|
| `create_membership_plan` | `(pool, req) -> MembershipPlan` | Default earn_rate_multiplier 1.5 |
| `get_membership_plans` | `(pool, merchant_id) -> Vec<MembershipPlan>` | WHERE is_active, ORDER BY price |
| `get_membership_plan` | `(pool, id) -> MembershipPlan` | 404 if missing |
| `subscribe_customer` | `(pool, merchant_id, customer_id, plan_id, expires_at) -> CustomerMembership` | INSERT with status 'active' |
| `get_customer_membership` | `(pool, merchant_id, customer_id) -> Option<CustomerMembership>` | Latest by created_at |
| `get_customer_membership_by_id` | `(pool, membership_id) -> CustomerMembership` | 404 if missing |
| `renew_membership` | `(pool, membership_id, new_expires_at) -> CustomerMembership` | Increment renewed_count, set status 'active' |
| `cancel_membership` | `(pool, membership_id) -> CustomerMembership` | Set status 'cancelled', cancelled_at = now() |
| `expire_membership` | `(pool, membership_id) -> CustomerMembership` | Set status 'expired' |
| `get_expired_memberships` | `(pool) -> Vec<CustomerMembership>` | WHERE status = 'active' AND expires_at < now() |

## Redemption (`src/services/redemption/storage.rs`)

| Function | Signature | Notes |
|----------|-----------|-------|
| `create_redemption` | `(pool, merchant_id, wallet_id, order_id, order_amount, payment_method, requested_amount) -> RedemptionRequest` | Initial state 'initiated' |
| `update_redemption_state` | `(pool, id, state, eligible_amount, applied_amount, debit_entry_id, compensation_entry_id, shopify_discount_id, rejection_reason) -> RedemptionRequest` | COALESCE for optional fields, updated_at = now() |
| `get_redemption` | `(pool, id) -> RedemptionRequest` | 404 if missing |
| `get_wallet_policy` | `(pool, merchant_id, bucket_type) -> Option<WalletPolicy>` | WHERE is_active = true |
| `get_wallet_policies` | `(pool, merchant_id) -> Vec<WalletPolicy>` | WHERE is_active = true |

## COD (`src/services/cod/storage.rs`)

| Function | Signature | Notes |
|----------|-----------|-------|
| `create_cod_order` | `(pool, merchant_id, order_id, wallet_id, ledger_entry_id) -> CodOrder` | State 'pending', unique violation -> Conflict |
| `get_cod_order_by_order_id` | `(pool, merchant_id, order_id) -> CodOrder` | 404 if missing |
| `update_cod_state` | `(pool, id, state, released_entry_id, cancelled_entry_id, delivery_confirmed_at) -> CodOrder` | COALESCE for optional fields |
| `get_pending_cod_orders` | `(pool, merchant_id, state_filter, page, limit) -> Vec<CodOrder>` | Optional state filter, paginated |
| `get_cod_analytics` | `(pool, merchant_id) -> CodAnalytics` | Aggregation query with FILTER clauses joining cod_orders + ledger_entries |
