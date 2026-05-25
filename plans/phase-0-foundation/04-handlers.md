# Phase 0: Handlers -- COMPLETED

All handlers use `#[tracing::instrument(skip(app_state))]`. Read handlers use `db_reader` when available.

## Ledger Handlers

`src/services/ledger/handler.rs`:
- `create_entry` -- POST, generates idempotency key if not provided, returns 201
- `get_entries` -- GET, paginated with bucket/movement filters, reads from replica
- `get_balance` -- GET, aggregated balance with per-bucket breakdown, reads from replica
- `get_balance_at` -- GET, point-in-time balance, reads from replica
- `get_entry_detail` -- GET, enriched view with customer/rule/campaign/event joins, reads from replica

## Wallet Handlers

`src/services/wallets/handler.rs`:
- `create_wallet` -- POST, returns WalletResponse with balance: None
- `get_wallet` -- GET by ID
- `lookup_wallet` -- GET by merchant_id + customer_id query params
- `get_or_create_wallet` -- POST, upsert semantics
- `list_wallets_for_merchant` -- GET, paginated (default 20, max 100)

## Identity Handlers

`src/services/identity/handler.rs`:
- `resolve_identity` -- POST, returns 201 for new customers, 200 for existing
- `get_customer` -- GET by ID, reads from replica
- `update_customer` -- PUT, partial update
- `search_customers` -- GET by phone and/or external_id, reads from replica

## Events Handlers

`src/services/events/handler.rs`:
- `ingest_event` -- POST, returns EventResponse with is_duplicate flag
- `shopify_order_webhook` -- POST, parses Shopify payload, creates "order.completed" event
- `get_event` -- GET by ID, reads from replica
- `list_events` -- GET, paginated with optional filters, reads from replica

## Earn Handlers

`src/services/earn/handler.rs` -- 20 handler functions:
- `process_earn` -- POST /earn/process
- `manual_credit` -- POST /earn/manual-credit, returns 201
- `birthday_bonus` -- POST /earn/birthday-bonus
- `create_milestone` -- POST /earn/milestones, returns 201
- `list_milestones` -- GET /earn/milestones/{merchant_id}
- `check_milestones` -- POST /earn/check-milestones
- `get_customer_milestones` -- GET /earn/milestones/{merchant_id}/{customer_id}
- `newsletter_signup` -- POST /earn/newsletter-signup, 201 for new, 200 for existing
- `get_newsletter_signup_count` -- GET /earn/newsletter-signups/{merchant_id}
- `profile_completion` -- POST /earn/profile-completion
- `create_streak_config` -- POST /earn/streaks, returns 201
- `list_streak_configs` -- GET /earn/streaks/{merchant_id}
- `check_streaks` -- POST /earn/check-streaks
- `create_wheel_config` -- POST /earn/spin-wheel/config, returns 201
- `get_wheel_config` -- GET /earn/spin-wheel/{merchant_id}
- `spin_wheel` -- POST /earn/spin-wheel/spin
- `create_membership_plan` -- POST /earn/memberships/plans, returns 201
- `list_membership_plans` -- GET /earn/memberships/plans/{merchant_id}
- `subscribe_membership` -- POST /earn/memberships/subscribe
- `renew_membership` -- POST /earn/memberships/renew
- `cancel_membership` -- POST /earn/memberships/cancel/{membership_id}
- `membership_status` -- GET /earn/memberships/status/{merchant_id}/{customer_id}

## Redemption Handlers

`src/services/redemption/handler.rs`:
- `initiate_redemption` -- POST, creates redemption and executes full state machine, returns 201
- `get_redemption` -- GET by ID, reads from replica
- `compensate_redemption` -- POST, reverses debits
- `check_eligibility` -- GET, evaluates per-bucket eligibility for a wallet, reads from replica

## COD Handlers

`src/services/cod/handler.rs`:
- `delivery_webhook` -- POST, dispatches to delivery or RTO processing based on status
- `cod_to_prepaid` -- POST, processes COD-to-prepaid incentive
- `list_cod_orders` -- GET, paginated with optional state filter, reads from replica
- `cod_analytics` -- GET, aggregate counts and amounts by state, reads from replica
