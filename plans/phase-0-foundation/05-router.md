# Phase 0: Routes -- COMPLETED

All routes registered in `src/main.rs` via `get_router()`. Each service provides its own `router()` function merged with `.with_state(app_state.clone())`.

## Health Check
- `GET /health` -- Returns `{"status": "ok"}`

## Ledger Routes (`src/services/ledger/mod.rs`)
- `POST /entries` -- Create ledger entry
- `GET /entries/{entry_id}` -- Get enriched entry detail
- `GET /wallets/{wallet_id}/entries` -- List entries for wallet (paginated, filterable)
- `GET /wallets/{wallet_id}/balance` -- Current balance with bucket breakdown
- `GET /wallets/{wallet_id}/balance/at` -- Point-in-time balance

## Wallet Routes (`src/services/wallets/mod.rs`)
- `POST /wallets` -- Create wallet
- `GET /wallets/{id}` -- Get wallet by ID
- `GET /wallets/lookup` -- Lookup by merchant_id + customer_id
- `POST /wallets/get-or-create` -- Upsert wallet
- `GET /merchants/{merchant_id}/wallets` -- List wallets for merchant

## Identity Routes (`src/services/identity/mod.rs`)
- `POST /identity/resolve` -- Resolve or create customer
- `GET /identity/customers/{id}` -- Get customer
- `PUT /identity/customers/{id}` -- Update customer
- `GET /identity/customers` -- Search by phone/external_id

## Events Routes (`src/services/events/mod.rs`)
- `POST /events/ingest` -- Ingest generic event
- `POST /events/shopify/orders` -- Shopify order webhook
- `GET /events/{id}` -- Get event
- `GET /events` -- List events (filterable)

## Earn Routes (`src/services/earn/mod.rs`)
- `POST /earn/process` -- Process order-based earn
- `POST /earn/manual-credit` -- Manual credit
- `POST /earn/birthday-bonus` -- Batch birthday bonus
- `POST /earn/milestones` -- Create milestone config
- `GET /earn/milestones/{merchant_id}` -- List milestones
- `POST /earn/check-milestones` -- Check and award milestones
- `GET /earn/milestones/{merchant_id}/{customer_id}` -- Customer's achieved milestones
- `POST /earn/newsletter-signup` -- Newsletter signup reward
- `GET /earn/newsletter-signups/{merchant_id}` -- Signup count
- `POST /earn/profile-completion` -- Profile completion reward
- `POST /earn/streaks` -- Create streak config
- `GET /earn/streaks/{merchant_id}` -- List streak configs
- `POST /earn/check-streaks` -- Check and award streaks
- `POST /earn/spin-wheel/config` -- Create spin wheel
- `GET /earn/spin-wheel/{merchant_id}` -- Get wheel config + segments
- `POST /earn/spin-wheel/spin` -- Spin the wheel
- `POST /earn/memberships/plans` -- Create membership plan
- `GET /earn/memberships/plans/{merchant_id}` -- List plans
- `POST /earn/memberships/subscribe` -- Subscribe to plan
- `POST /earn/memberships/renew` -- Renew membership
- `POST /earn/memberships/cancel/{membership_id}` -- Cancel membership
- `GET /earn/memberships/status/{merchant_id}/{customer_id}` -- Membership status

## Redemption Routes (`src/services/redemption/mod.rs`)
- `POST /redemptions` -- Initiate and execute redemption
- `GET /redemptions/{id}` -- Get redemption
- `POST /redemptions/{id}/compensate` -- Compensate failed redemption
- `GET /wallets/{wallet_id}/eligibility` -- Check redemption eligibility

## COD Routes (`src/services/cod/mod.rs`)
- `POST /cod/webhook/delivery` -- Delivery/RTO webhook
- `POST /cod/incentive` -- COD-to-prepaid incentive
- `GET /cod/orders/{merchant_id}` -- List COD orders
- `GET /cod/analytics/{merchant_id}` -- COD analytics

## Additional Services (routers registered in main.rs)
These services were scaffolded in Phase 0 and are registered in `get_router()`:
- `services::rules::router()` -- Rules engine
- `services::notifications::router()` -- Notification templates and logs
- `services::campaigns::router()` -- Campaign management
- `services::loyalty::router()` -- Loyalty programs and tiers
- `services::gift_cards::router()` -- Gift card lifecycle
- `services::referrals::router()` -- Referral programs and conversions
- `services::admin::router()` -- Admin endpoints
