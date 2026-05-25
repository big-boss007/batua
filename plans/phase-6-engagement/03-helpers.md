# Phase 6: Helpers

All in `src/services/earn/helpers.rs` unless noted.

## Birthday Bonus
- `process_birthday_bonuses(pool, merchant_id, amount)` -- Calls `identity::storage::get_customers_with_birthday_today`, iterates each customer, creates EarnedCredit ledger entry. Idempotency: SHA-256 of merchant_id + customer_id + "birthday" + today's date. Returns processed/credited/skipped counts.

## Milestones
- `check_and_award_milestones(pool, merchant_id, customer_id)` -- Fetches active milestones and customer order stats. For each unachieved milestone, checks if threshold is crossed (order_count or lifetime_spend). Awards credit via ledger entry, records achievement. Idempotency: SHA-256 of merchant_id + customer_id + milestone_id.

## Newsletter Signup
- `process_newsletter_signup(pool, req)` -- Validates email via `identity::helpers::validate_email`. Resolves customer (by customer_id, phone, or email). Checks for existing signup. Creates EarnedCredit entry, records signup. Idempotency: SHA-256 of merchant_id + customer_id + "newsletter_signup".
- `resolve_newsletter_customer(pool, req)` -- Resolution priority: customer_id -> phone (resolve_or_create) -> email lookup. Returns error if no identifier resolves.

## Profile Completion
- `process_profile_completion(pool, req)` -- Checks name, email, birthday fields. If all filled: creates 30.0 credit EarnedCredit entry. Idempotency: SHA-256 of merchant_id + customer_id + "profile_complete". Returns completion percentage, field lists, reward status.

## Streaks
- `check_and_award_streaks(pool, merchant_id, customer_id)` -- Fetches active streak configs, counts recent orders within each window. Calculates progress percentage. If orders_in_window >= required_orders and no prior achievement in same window: creates EarnedCredit entry, records achievement. Idempotency: SHA-256 of merchant_id + customer_id + config_id + window_start_date. Returns both achieved streaks and active streak progress.

## Spin Wheel
- `create_wheel(pool, req)` -- Creates wheel config (default name "Lucky Wheel", limit 1), inserts segments in position order (default color #7c6aff).
- `spin_wheel(pool, req)` -- Validates wheel active, checks daily limit, loads segments, runs probability-weighted selection (UUID-based random), creates EarnedCredit entry for non-zero rewards, records result. Returns winning segment + remaining spins.

## Memberships
- `subscribe_to_plan(pool, req)` -- Validates plan active, checks for existing active subscription to same plan. Calculates expiry (30d monthly, 365d annual). Returns is_new flag.
- `renew_membership(pool, req)` -- Extends from max(current_expires_at, now()) + plan duration. Increments renewed_count.
- `get_membership_status(pool, merchant_id, customer_id)` -- Fetches latest membership. Auto-expires if active but past expires_at. Returns days_remaining.
- `cancel_membership_by_id(pool, membership_id)` -- Delegates to storage::cancel_membership.

## Coalition (in `src/services/admin/helpers.rs`)
- `transfer_coalition_credits(pool, req)` -- Validates shared coalition membership, checks sender balance (spendable_balance >= amount), calculates conversion rate (to_rate / from_rate), creates paired OUT/IN ledger entries via `create_across_movement`, records transfer. Returns balances after transfer.
