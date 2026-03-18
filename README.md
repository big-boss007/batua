# Batua (Breeze)

Wallet · Loyalty · Gift Cards · Passes — SaaS for Shopify D2C merchants, India-first.

## Quick Start

### Prerequisites
- Rust (1.85+, edition 2024)
- Node.js 22+
- PostgreSQL 16
- Redis 7

### Option A: Docker (recommended)
```bash
docker compose up -d
cp .env.docker .env
cargo run
```

### Option B: Local
```bash
# Start Postgres + Redis
brew services start postgresql@16
brew services start redis

# Create database
createdb batua
createdb batua_test

# Run migrations
for f in $(ls migrations/*.sql | sort); do psql -d batua -f "$f"; done
for f in $(ls migrations/*.sql | sort); do psql -d batua_test -f "$f"; done

# Configure
cp .env.example .env

# Start backend
cargo run

# Start frontend (separate terminal)
cd frontend && npm install && npm run dev
```

### Seed Demo Data
```bash
make seed
```
Creates: 1 merchant (Desi Threads), 8 customers, 20 orders, 3 reward rules, loyalty tiers, referral codes, notification templates.

## URLs

| App | URL | Audience |
|-----|-----|----------|
| Super Admin | http://localhost:5174/platform | Breeze team |
| Merchant Admin | http://localhost:5174/admin | Merchant |
| Customer Storefront | http://localhost:5174/s/desi-threads | End customer |
| API | http://localhost:3000 | Backend |
| Health Check | http://localhost:3000/health | Ops |

## Commands

```bash
make check          # cargo check
make test           # cargo test (unit + integration + lint)
make run            # cargo run
make dev            # Start backend + frontend together
make stop           # Stop all services
make seed           # Seed demo data
make reset-and-seed # Drop DB, recreate, migrate, seed
make fmt            # cargo fmt
```

Frontend:
```bash
cd frontend
npm run dev          # Dev server
npm run check        # Type check
npm run format       # Prettier
npm run build        # Production build
```

## Architecture

- **Backend**: Rust / Axum 0.8 / PostgreSQL / Redis
- **Frontend**: SvelteKit / Svelte 5 / @juspay/svelte-ui-components
- **15 backend services**: ledger, wallets, events, rules, identity, earn, redemption, COD, notifications, campaigns, loyalty, gift cards, referrals, admin
- **60 tests**: 33 unit + 26 integration + 1 tracing lint
- **19 database tables** enforcing 8 foundational ledger truths

See `docs/architecture-diagrams.html` for visual diagrams.

## API Docs

All endpoints documented in `docs/api-*.md`:
- [Admin](docs/api-admin.md) | [Identity](docs/api-identity.md) | [Wallets](docs/api-wallets.md) | [Ledger](docs/api-ledger.md)
- [Events](docs/api-events.md) | [Rules](docs/api-rules.md) | [Earn](docs/api-earn.md) | [Redemption](docs/api-redemption.md)
- [COD](docs/api-cod.md) | [Notifications](docs/api-notifications.md) | [Loyalty](docs/api-loyalty.md)
- [Gift Cards](docs/api-gift-cards.md) | [Referrals](docs/api-referrals.md)

## Testing

```bash
# All tests
cargo test

# Integration tests only (needs DB)
cargo test --test integration_tests

# UAT script
bash scripts/uat.sh
```

## Project Structure

```
src/
├── main.rs              # Entry point, router composition
├── app_state.rs         # AppState (PgPool, Redis)
├── error.rs             # AppError enum
├── services/            # 15 service modules
│   ├── ledger/          # Core ledger (8 truths)
│   ├── wallets/         # Wallet CRUD + balance
│   ├── events/          # Event ingestion + Shopify webhook
│   ├── rules/          # DSL rule engine
│   ├── earn/            # Earn flow orchestrator
│   ├── redemption/      # Redemption engine
│   └── ...
frontend/
├── src/routes/
│   ├── admin/           # Merchant admin (11 pages)
│   ├── platform/        # Super admin (5 pages)
│   └── s/[slug]/        # Customer storefront (5 pages)
├── src/lib/client/modules/  # Feature modules
migrations/              # 19 SQL migrations
docs/                    # API docs + architecture diagrams
scripts/                 # seed.sh, uat.sh
```
