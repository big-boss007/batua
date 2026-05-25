# Phase 0: Database Schema -- COMPLETED

26 migrations in `migrations/`, all using sqlx. PostgreSQL 15+ required.

## Migration Index

### Batch 1: Core Schema (2026-03-18)

| # | Migration | Tables / Objects |
|---|-----------|-----------------|
| 001 | `core_enums` | Types: `movement_type` (in/held/out/across), `actor_type` (system/human/automation/migration), `bucket_type` (8 variants), `credit_state` (5 variants), `redemption_state` (8 variants), `event_state` (5 variants) |
| 002 | `merchants` | `merchants` -- external_id (Shopify shop ID), name, domain, currency (INR), timezone (Asia/Kolkata), is_active |
| 003 | `customers` | `customers` -- phone (E.164, unique), email, name, external_id, is_verified |
| 004 | `wallets` | `wallets` -- merchant_id FK, customer_id FK (nullable for bearer), is_bearer, bearer_code (unique), UNIQUE(merchant_id, customer_id) |
| 005 | `ledger_entries` | `ledger_entries` -- immutable append-only ledger with earning_unit/currency_equivalent/conversion_rate triple, idempotency_key (unique), full cause chain, constraints JSONB, expires_at. Trigger `prevent_ledger_mutation` blocks updates to immutable columns. 8 indexes. |
| 006 | `events` | `events` -- merchant_id FK, event_type, event_source, external_event_id, payload JSONB, state, idempotency_key (unique). 5 indexes. |
| 007 | `rules_and_snapshots` | `rules` -- versioned rule definitions with config JSONB. `rule_snapshots` -- immutable frozen configs. `campaigns` -- time-bounded overlays with multipliers. `campaign_snapshots` -- immutable campaign configs. |
| 008 | `wallet_policies` | `wallet_policies` -- per-merchant per-bucket constraints: min_redemption, step_size, max_per_order_pct/fixed, stackable_with_discounts, default_conversion_rate, default_expiry_days, is_transferable, excluded_payment_methods[], excluded_collections[]. UNIQUE(merchant_id, bucket_type). |
| 009 | `redemption_requests` | `redemption_requests` -- state machine (redemption_state), requested/eligible/applied amounts, order context, linked debit/compensation entries, shopify_discount_id. |
| 010 | `connectors` | `connectors` -- capability->vendor mapping (whatsapp-bsp, sms, email, payment-gateway), config JSONB, priority for fallback. UNIQUE(merchant_id, capability, vendor). |
| 011 | `notifications` | `notification_templates` -- channel (whatsapp/sms/email), locale, template_id, body_template, variables. `notification_logs` -- delivery tracking with status lifecycle. |
| 012 | `loyalty_programs` | `loyalty_programs` -- evaluation_criteria (spend/points/order_count), evaluation_period_days. `loyalty_tiers` -- ranked tiers with earn_rate_multiplier and benefits JSONB. `customer_tiers` -- current tier assignment with qualifying_value and expiry. |
| 013 | `gift_cards` | `gift_cards` -- initial/current amount, bearer wallet, code, batch support, claim tracking (claimed_by_wallet_id). |
| 014 | `referrals` | `referral_programs` -- per-merchant config with referrer/referee amounts. `referral_codes` -- vanity codes, creator mode, commission_rate. `referral_conversions` -- anti-fraud signals (IP, device fingerprint, is_suspicious). |
| 015 | `commerce_cache` | `product_collection_mappings` -- synced Shopify product-to-collection data. `customer_order_stats` -- aggregated order count/spend per merchant-customer. |
| 016 | `geo_policies` | `geo_policies` -- geo-specific behavior config. Adds `geo_policy_id` FK to merchants. |
| 017 | `cod_orders` | `cod_orders` -- merchant_id, order_id, wallet_id, ledger_entry_id, state (pending/delivered/rto/cancelled), delivery timestamps, released/cancelled entry IDs. UNIQUE(merchant_id, order_id). |
| 018 | `merchant_slugs` | Merchant URL slug support |
| 019 | `plan_tier` | Plan tier extensions |

### Batch 2: Earn Features (2026-03-19)

| # | Migration | Tables / Objects |
|---|-----------|-----------------|
| 001 | `customer_birthday` | Adds `birthday DATE` column to customers |
| 002 | `milestones` | `milestone_configs` -- merchant_id, name, milestone_type (order_count/lifetime_spend), threshold, reward_amount. `milestone_achievements` -- customer_id, milestone_id, ledger_entry_id. UNIQUE(customer_id, milestone_id). |
| 003 | `newsletter_signups` | `newsletter_signups` -- merchant_id, customer_id, email, source, ledger_entry_id. UNIQUE(merchant_id, customer_id). |
| 004 | `streaks` | `streak_configs` -- required_orders, window_days, reward_amount. `streak_achievements` -- window_start/end, ledger_entry_id. |
| 005 | `spin_wheel` | `spin_wheel_configs` -- daily_spin_limit. `spin_wheel_segments` -- label, reward_amount, probability, color, position. `spin_results` -- per-spin tracking. |
| 006 | `memberships` | `membership_plans` -- plan_type, price, earn_rate_multiplier, benefits JSONB. `customer_memberships` -- status, started_at, expires_at, renewed_count, cancelled_at. |
| 007 | `coalition` | Coalition/multi-merchant support |

## Key Design Decisions

- **Immutability enforced at DB level**: The `prevent_ledger_mutation` trigger on `ledger_entries` raises an exception if any column other than `state` and `expires_at` is modified
- **Idempotency everywhere**: `ledger_entries.idempotency_key` and `events.idempotency_key` are UNIQUE -- all writes are safe to retry
- **One wallet per customer per merchant**: Enforced by `UNIQUE(merchant_id, customer_id)` on wallets
- **PostgreSQL custom types**: All enums are Postgres ENUM types, mapped to Rust via `sqlx::Type`
- **JSONB for flexible data**: constraints, config, payload, benefits, variables, fraud_signals
- **Array columns**: `excluded_payment_methods TEXT[]`, `excluded_collections TEXT[]` on wallet_policies
- **Nullable FKs**: customer_id on wallets (bearer instruments), event_id/rule_snapshot_id/campaign_snapshot_id on ledger_entries
