# Phase 0: Storage Layer -- COMPLETED

All storage functions use sqlx with `#[tracing::instrument(skip(pool), err(Debug))]`. Read-only queries support the optional reader replica via `app_state.db_reader.as_ref().unwrap_or(&app_state.db)` in handlers.

## Ledger Storage

`src/services/ledger/storage.rs`:
- `create_entry` -- INSERT with idempotency (checks existing by key first, returns existing if found)
- `create_across_movement` -- Transactional pair (OUT + IN) with shared transfer_id, ON CONFLICT upsert
- `get_entries` -- Paginated query with optional bucket_type/movement_type filters
- `get_balance` -- Aggregates by bucket_type + movement_type for active, non-expired entries; builds displayed/spendable per bucket
- `get_balance_at` -- Same as get_balance but with a point-in-time cutoff
- `entry_exists_by_idempotency_key` -- Existence check for idempotency deduplication
- `get_entry_detail` -- Enriched query joining wallets -> customers, rule_snapshots -> rules, campaign_snapshots -> campaigns, events, and linked entries by transfer_id
- Internal: `build_wallet_balance` -- HashMap-based bucket aggregation (In adds to both displayed+spendable, Held adds to displayed only, Out subtracts, Across is neutral)

## Wallet Storage

`src/services/wallets/storage.rs`:
- `create_wallet` -- INSERT with unique violation -> Conflict error
- `get_wallet` -- By ID, 404 if missing
- `get_wallet_by_merchant_customer` -- Optional lookup
- `get_or_create_wallet` -- INSERT ON CONFLICT DO NOTHING + fallback SELECT
- `get_bearer_wallet` -- Lookup by bearer_code where is_bearer = true
- `list_wallets_for_merchant` -- Paginated, ordered by created_at DESC

## Identity Storage

`src/services/identity/storage.rs`:
- `resolve_by_email` -- Optional lookup by email
- `resolve_by_phone` -- Optional lookup by normalized phone
- `create_customer` -- INSERT with email validation, unique violation -> Conflict
- `get_customer` -- By ID, 404 if missing
- `update_customer` -- COALESCE-based partial update (only non-null fields overwrite)
- `resolve_or_create` -- Phone-first: check existing, then INSERT ON CONFLICT DO NOTHING + fallback SELECT
- `search_customers` -- By phone and/or external_id (at least one required)
- `get_customers_with_birthday_today` -- JOIN wallets, filter by EXTRACT(MONTH/DAY) matching current date

## Events Storage

`src/services/events/storage.rs`:
- `store_event` -- Idempotent insert (SHA256 of merchant_id:source:external_id), returns (Event, is_duplicate)
- `get_event` -- By ID, 404 if missing
- `mark_event_state` -- UPDATE state + optional processed_at
- `get_pending_events` -- WHERE state = Received, ordered by created_at ASC
- `list_events` -- Paginated with optional merchant_id, event_type, state filters using nullable parameter patterns

## Earn Storage

`src/services/earn/storage.rs`:
- `get_customer_order_stats` / `update_order_stats` -- UPSERT on customer_order_stats
- Milestones: `create_milestone_config`, `get_active_milestones`, `has_achieved_milestone`, `record_milestone_achievement`, `get_customer_milestones`
- Newsletter: `has_newsletter_signup`, `record_newsletter_signup`, `get_newsletter_signup_count`
- Streaks: `create_streak_config`, `get_active_streak_configs`, `count_recent_orders` (distinct payment_reference in window), `has_streak_achievement_in_window`, `record_streak_achievement`
- Spin wheel: `create_wheel_config`, `create_wheel_segment`, `get_wheel_config`, `get_wheel_segments`, `count_spins_today`, `record_spin_result`
- Memberships: `create_membership_plan`, `get_membership_plans`, `get_membership_plan`, `subscribe_customer`, `get_customer_membership`, `get_customer_membership_by_id`, `renew_membership`, `cancel_membership`, `expire_membership`, `get_expired_memberships`

## Redemption Storage

`src/services/redemption/storage.rs`:
- `create_redemption` -- INSERT with state = 'initiated'
- `update_redemption_state` -- COALESCE-based partial update for state transitions
- `get_redemption` -- By ID, 404 if missing
- `get_wallet_policy` / `get_wallet_policies` -- Active policies by merchant, optionally filtered by bucket_type

## COD Storage

`src/services/cod/storage.rs`:
- `create_cod_order` -- INSERT with unique violation handling
- `get_cod_order_by_order_id` -- By merchant_id + order_id
- `update_cod_state` -- COALESCE-based state transition with optional released/cancelled entry IDs
- `get_pending_cod_orders` -- Paginated, optional state filter
- `get_cod_analytics` -- Aggregate query using FILTER(WHERE) for pending/delivered/rto counts and amounts
