# Integration

## Objective

Wire all copied pieces into a working standalone repo. This phase
captures what gets *adapted* during the port (where files are not pure
copies) and how the new repo's root-level orchestration differs from the
monorepo it came from.

## Tasks

### File-level copies (no adaptation)

- `Cargo.toml`, `Cargo.lock`
- `src/` (recursive)
- `migrations/` (recursive)
- `tests/` (recursive)
- `scripts/seed.sh`, `scripts/seed.sql`, `scripts/seed-full.sql`,
  `scripts/seed-large.sql`, `scripts/seed-large-phase6.sql`,
  `scripts/uat.sh`
- `docker-compose.yml`
- `.env`, `.env.docker`, `.env.example`
- `Backend_Dev_Rules.md`
- `docs/api-admin.md`, `docs/api-cod.md`, `docs/api-earn.md`,
  `docs/api-events.md`, `docs/api-gift-cards.md`, `docs/api-identity.md`,
  `docs/api-ledger.md`, `docs/api-loyalty.md`,
  `docs/api-notifications.md`, `docs/api-redemption.md`,
  `docs/api-referrals.md`, `docs/api-rules.md`, `docs/api-wallets.md`
- `docs/storefront-home-api.md` (referenced as the documentation gold
  standard by `CLAUDE.md`, kept so the rule remains actionable)

### File-level *adaptations*

- **`.gitignore`**: drop the `site/node_modules` and `site/test-results`
  entries since the `site/` directory is not ported. Keep everything else.
- **`Makefile`**: drop the `dev` target's `cd frontend && npm run dev`
  line and the `pkill -f "vite dev"` line from `stop`. Keep
  `check`/`test`/`run`/`fmt`/`seed`/`reset-db`/`reset-and-seed`.
- **`README.md`**: trim "Option B: Local" frontend lines (`cd frontend &&
  npm install && npm run dev`); remove the frontend URL row. Keep all
  backend Quick Start and seed sections intact.
- **`CLAUDE.md`**: keep the Backend section verbatim, drop the Frontend
  section, leave the Shared Conventions section intact.
- **`docker-compose.yml`**: copied verbatim — it is backend-only already.

### Excluded entirely (not copied)

- `frontend/`, `site/`, `target/`, `e2e-report/`, `.playwright-mcp/`
- Repo-root screenshots (`*.png`)
- `docs/` HTML, PDF, screenshot, marketing, and audit folders
- Other plans in `plans/` not relevant to backend-only operation

## Outputs

- A buildable `batua-be/` repo with adapted Makefile/README/CLAUDE.md/.gitignore.
- `diff -rq` between the source and ported `src/`, `migrations/`, `tests/`,
  `Cargo.toml`, `Cargo.lock` returns no differences.

## Validation

```bash
cd /Users/chirag/Developer/batua-be
cargo check
cargo test
cargo run            # boots without panic, then Ctrl-C
```

All three must succeed (or `cargo test` must show the *same* result as in
the source repo — see overview success criterion #6).
