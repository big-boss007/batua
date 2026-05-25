# Phase 0: Foundation -- Checklist

All items complete.

## Infrastructure
- [x] Axum 0.8 server with graceful startup
- [x] PostgreSQL connection pool (writer, max 20)
- [x] Optional PostgreSQL reader replica pool
- [x] Redis client connection
- [x] AppState struct with db, db_reader, redis
- [x] Environment variable configuration (DATABASE_URL, DATABASE_READER_URL, REDIS_URL, PORT, LOG_FORMAT)
- [x] Structured logging (JSON/pretty) via tracing_subscriber
- [x] CORS middleware (permissive)
- [x] Request ID generation and propagation (x-request-id)
- [x] HTTP trace layer
- [x] Health check endpoint (GET /health)
- [x] Unified error handling (AppError -> JSON response with status codes)

## Database Schema
- [x] Core enum types (movement_type, actor_type, bucket_type, credit_state, redemption_state, event_state)
- [x] merchants table with Shopify external_id
- [x] customers table with phone uniqueness (E.164)
- [x] wallets table with one-per-customer-per-merchant constraint
- [x] ledger_entries table with immutability trigger
- [x] events table with idempotency
- [x] rules + rule_snapshots + campaigns + campaign_snapshots
- [x] wallet_policies table
- [x] redemption_requests table with state machine
- [x] connectors table
- [x] notification_templates + notification_logs
- [x] loyalty_programs + loyalty_tiers + customer_tiers
- [x] gift_cards table
- [x] referral_programs + referral_codes + referral_conversions
- [x] product_collection_mappings + customer_order_stats
- [x] geo_policies table
- [x] cod_orders table
- [x] customer birthday column
- [x] milestone_configs + milestone_achievements
- [x] newsletter_signups table
- [x] streak_configs + streak_achievements
- [x] spin_wheel_configs + spin_wheel_segments + spin_results
- [x] membership_plans + customer_memberships
- [x] All 26 migrations created

## Ledger Service
- [x] mod.rs with router (5 routes)
- [x] types.rs (4 enums, 9 structs)
- [x] handler.rs (5 handlers with tracing)
- [x] storage.rs (7 functions: create_entry, create_across_movement, get_entries, get_balance, get_balance_at, entry_exists_by_idempotency_key, get_entry_detail)
- [x] helpers.rs (generate_idempotency_key, validate_double_entry)
- [x] Idempotency on all writes
- [x] Reader replica on all reads

## Wallet Service
- [x] mod.rs with router (5 routes)
- [x] types.rs (7 structs)
- [x] handler.rs (5 handlers with tracing)
- [x] storage.rs (6 functions)
- [x] helpers.rs (generate_bearer_code)
- [x] Unique constraint handling (Conflict errors)
- [x] Upsert semantics (get_or_create_wallet)

## Identity Service
- [x] mod.rs with router (4 routes)
- [x] types.rs (5 structs)
- [x] handler.rs (4 handlers with tracing)
- [x] storage.rs (8 functions including resolve_or_create, get_customers_with_birthday_today)
- [x] helpers.rs (normalize_phone, validate_email with unit tests)
- [x] Phone normalization for India (+91)
- [x] Reader replica on reads

## Events Service
- [x] mod.rs with router (4 routes)
- [x] types.rs (1 enum, 8 structs)
- [x] handler.rs (4 handlers with tracing)
- [x] storage.rs (5 functions)
- [x] helpers.rs (idempotency key, Shopify parsing, payment method extraction, COD detection)
- [x] Idempotent event ingestion
- [x] Shopify order webhook support

## Earn Service
- [x] mod.rs with router (22 routes)
- [x] types.rs (35+ structs covering all earn mechanics)
- [x] handler.rs (22 handlers with tracing)
- [x] storage.rs (30+ functions covering milestones, newsletter, streaks, spin wheel, memberships)
- [x] helpers.rs (full earn orchestration: order processing, manual credit, birthday bonus, newsletter, profile completion, milestones, streaks, spin wheel, memberships)
- [x] Cross-service integration (events, identity, wallets, ledger, rules, cod)

## Redemption Service
- [x] mod.rs with router (4 routes)
- [x] types.rs (1 enum, 11 structs)
- [x] handler.rs (4 handlers with tracing)
- [x] storage.rs (5 functions)
- [x] helpers.rs (eligibility evaluation, constraint validation, state machine execution, compensation)
- [x] Per-bucket policy enforcement
- [x] Stub Shopify discount application

## COD Service
- [x] mod.rs with router (4 routes)
- [x] types.rs (1 enum, 8 structs)
- [x] handler.rs (4 handlers with tracing)
- [x] storage.rs (5 functions)
- [x] helpers.rs (delivery processing, RTO processing, incentive calculation, COD-to-prepaid)
- [x] Across movement for delivery confirmation (CodPending -> EarnedCredit)
- [x] Analytics aggregation

## Service Integration
- [x] All 14 service modules declared in src/services/mod.rs
- [x] All routers merged in src/main.rs::get_router()
- [x] tracing::instrument on all pub async fn in services
