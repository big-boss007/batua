# Phase 0: Foundation -- COMPLETED

**Status:** COMPLETED
**Started:** 2026-03-18
**Completed:** 2026-03-19

## Goal

Build the core backend services for Batua -- a wallet/loyalty/gift-card platform for Shopify D2C merchants, India-first. Establish the immutable ledger, wallet management, identity resolution, event ingestion, earn/burn flows, COD handling, and all supporting database schema.

## Scope

- Immutable append-only ledger with bucket-typed credits (8 bucket types, 4 movement types)
- Wallet service with one-wallet-per-customer-per-merchant invariant and bearer wallet support
- Identity service with phone-first resolution (India +91 normalization) and upsert semantics
- Event ingestion with Shopify order webhook support and idempotent processing
- Earn service: order-based earning, manual credits, birthday bonuses, milestones, streaks, newsletter signup rewards, profile completion rewards, spin-the-wheel, paid memberships
- Redemption service: eligibility evaluation, per-bucket policy enforcement, state machine (initiated -> validating -> committed -> applied -> completed), compensation flow
- COD service: held credits for COD orders, delivery confirmation -> release, RTO -> cancellation, COD-to-prepaid incentives, analytics
- Rules engine with immutable snapshots and campaign overlays
- 26 database migrations covering all tables, enums, triggers, and indexes
- Infrastructure: Axum 0.8, sqlx 0.8, PostgreSQL (with optional reader replica), Redis, request ID propagation, structured logging (JSON/pretty), CORS

## Success Criteria

- [x] All 7 core services (ledger, wallets, identity, events, earn, redemption, cod) have handler/storage/helpers/types
- [x] Ledger entries are immutable (enforced by DB trigger `prevent_ledger_mutation`)
- [x] Idempotency keys on all write paths prevent duplicate processing
- [x] Reader replica support on all read handlers
- [x] Every `pub async fn` in services has `#[tracing::instrument]`
- [x] All 26 migrations run cleanly and create the full schema
- [x] AppState holds writer pool, optional reader pool, and Redis client

## Dependencies

- PostgreSQL 15+
- Redis
- Rust (Edition 2024), Axum 0.8, sqlx 0.8, tokio
- tower-http (CORS, trace, request ID)
- serde, serde_json, chrono, uuid, sha2, hex, thiserror
