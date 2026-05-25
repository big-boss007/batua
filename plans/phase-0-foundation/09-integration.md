# Phase 0: Integration -- COMPLETED

How all services wire together in `src/main.rs`.

## Application Bootstrap

`src/main.rs::main()`:
1. Load `.env` via dotenvy
2. Initialize tracing subscriber (JSON or pretty based on `LOG_FORMAT`)
3. Connect to PostgreSQL writer (`DATABASE_URL`, required, max 20 connections)
4. Optionally connect to PostgreSQL reader (`DATABASE_READER_URL`, max 20 connections)
5. Connect to Redis (`REDIS_URL`, defaults to localhost:6379)
6. Construct `AppState { db, db_reader, redis }`
7. Build router via `get_router(app_state)`
8. Apply middleware layers (Trace, CORS, RequestId)
9. Bind to `0.0.0.0:{PORT}` (default 3000)
10. Serve via `axum::serve`

## Router Assembly

`src/main.rs::get_router()` merges all service routers:
```
/health -> health_check
ledger::router()
wallets::router()
events::router()
rules::router()
identity::router()
earn::router()
redemption::router()
cod::router()
notifications::router()
campaigns::router()
loyalty::router()
gift_cards::router()
referrals::router()
admin::router()
```

Each service calls `.with_state(app_state.clone())` to inject the shared AppState.

## Service Module Registry

`src/services/mod.rs` declares all 14 service modules:
admin, campaigns, cod, earn, events, gift_cards, identity, ledger, loyalty, notifications, redemption, referrals, rules, wallets

## Cross-Service Dependencies

Services call each other's storage and helper modules directly (no HTTP):

- **earn** depends on: events (storage), identity (storage+helpers), wallets (storage), ledger (storage), rules (helpers), cod (storage)
- **redemption** depends on: wallets (storage), ledger (storage+types)
- **cod** depends on: ledger (storage+types), wallets (storage)
- **events** is standalone (no cross-service dependencies)
- **ledger** is standalone (no cross-service dependencies)
- **wallets** is standalone (no cross-service dependencies)
- **identity** is standalone (no cross-service dependencies)

## Environment Variables

| Variable | Required | Default | Purpose |
|----------|----------|---------|---------|
| `DATABASE_URL` | Yes | -- | PostgreSQL writer connection |
| `DATABASE_READER_URL` | No | -- | PostgreSQL reader replica |
| `REDIS_URL` | No | `redis://localhost:6379` | Redis connection |
| `PORT` | No | `3000` | HTTP listen port |
| `LOG_FORMAT` | No | `pretty` | `json` or `pretty` |
| `RUST_LOG` | No | `info` | Log level filter |
