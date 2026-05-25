# Backend Port: batua → batua-be

## Goal

Produce a standalone, self-contained backend repository at
`/Users/chirag/Developer/batua-be/` that is a **bit-for-bit** copy of the
current backend in `/Users/chirag/Developer/batua/` (Rust/Axum services,
migrations, tests, type generation outputs, scripts, docs, and config).
After the port the new repo must compile, pass tests, and run identically
to the source.

## Scope

### In scope

- `Cargo.toml`, `Cargo.lock`
- `src/` (entire tree — 78 `.rs` files across 14 services + `main.rs`,
  `lib.rs`, `app_state.rs`, `error.rs`, `helper.rs`, `generated/`)
- `migrations/` (all 30 SQL files)
- `tests/` (`functional_tests.rs`, `integration_tests.rs`,
  `ledger_edge_cases.rs`, `lint_tracing.rs`)
- `scripts/seed.sh`, `scripts/seed*.sql`, `scripts/uat.sh`
- `docker-compose.yml`
- `.env`, `.env.docker`, `.env.example`
- `.gitignore`
- `Backend_Dev_Rules.md`
- `CLAUDE.md` (trimmed to backend-only sections)
- `README.md` (trimmed to backend-only)
- `Makefile` (trimmed: drop the frontend `dev` orchestration but keep
  `check`, `test`, `run`, `fmt`, `seed`, `reset-db`, `reset-and-seed`)
- `docs/api-*.md` (the 14 backend API docs)
- `docs/storefront-home-api.md` (referenced as the doc-format gold standard)
- `plans/` (this plan only — other plans stay in source repo unless backend-only)

### Out of scope

- `frontend/` (entire SvelteKit app)
- `site/`
- All marketing assets (`*.png` at repo root, `docs/*.html`, `docs/*.pdf`,
  `docs/breeze-*`, `docs/admin-*`, `docs/audit/`, `docs/autoresearch-reports/`,
  `docs/UAT*`, screenshots etc.)
- `target/` build artifacts
- `.playwright-mcp/`, `e2e-report/`
- Frontend dev-rules and frontend-only plans

## Success criteria

1. `diff -rq` between `batua/src` and `batua-be/src` returns nothing.
2. `diff -rq` between `batua/migrations` and `batua-be/migrations` returns nothing.
3. `diff -rq` between `batua/tests` and `batua-be/tests` returns nothing.
4. `cmp batua/Cargo.toml batua-be/Cargo.toml` exits 0, same for `Cargo.lock`.
5. `cargo check` in `batua-be/` exits 0.
6. `cargo test` in `batua-be/` shows the **same** pass/fail count as in the
   original repo (test parity, not absolute success — we are porting, not
   fixing).
7. All 30 migrations apply cleanly on a fresh PostgreSQL database.
8. `cargo run` from `batua-be/` boots the server on the configured port
   without panicking.

## Approach

This is a **migration / extraction**, not a redesign. The canonical way to
guarantee bit-for-bit fidelity is to copy files directly (`cp -R`) rather
than re-derive them. The Skulls service-creation template is built around
"design a new service from scratch", so most of its phases (`01-types`
through `08-scheduler`) are marked SKIPPED — there is no new code to
design. The two phases that map onto this port are:

- `09-integration.md` — how the ported pieces fit together: Cargo, env,
  Makefile, docker-compose, scripts. This is where adaptations (Makefile
  frontend strip, README trim) get described.
- `10-database.md` — migration parity: identical SQL files must apply
  cleanly to a fresh database.

The real execution checklist lives in `checklist.md`.

## Dependencies

- Local PostgreSQL reachable as `chirag@localhost` (matches `.env`).
- Local Redis on `localhost:6379`.
- Disk space for a second `target/` build (Rust artifacts can be GBs).
- No network dependency for the copy itself.

## Risks

| Risk | Mitigation |
|------|------------|
| `target/` gets copied accidentally and bloats new repo | Explicit `cp` of named directories only; never `cp -R batua/* batua-be/` |
| `.env` carries secrets we shouldn't duplicate | `.env` here has no secrets (local Postgres user only); copy as-is and call it out |
| `Cargo.lock` drift if we run `cargo update` | Copy `Cargo.lock` verbatim; do not run `cargo update` |
| Tests rely on hard-coded paths from `batua/` | Verify with `cargo test` in new repo; flag any failures vs. source |
| Migrations leave residual state in shared DB | Use a throwaway DB `batua_be_verify` for migration check, then drop it |
