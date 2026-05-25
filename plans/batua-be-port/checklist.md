# Implementation Checklist

## Phase 0 — Plan & Approval

- [ ] All 12 plan files written to `plans/batua-be-port/`
- [ ] Plan summary shown to user
- [ ] User explicitly approves before any `cp` happens

## Phase 1 — Scaffold

- [ ] `mkdir -p /Users/chirag/Developer/batua-be/{src,migrations,tests,scripts,docs,plans}`
- [ ] Confirm `batua-be/` is empty before copy (do not overwrite existing work)

## Phase 2 — Copy (verbatim)

- [ ] `cp -R src/* batua-be/src/`
- [ ] `cp -R migrations/* batua-be/migrations/`
- [ ] `cp -R tests/* batua-be/tests/` (only the `.rs` files — skip the UAT report subfolders and screenshots in `tests/`)
- [ ] `cp Cargo.toml Cargo.lock batua-be/`
- [ ] `cp scripts/seed*.sql scripts/seed.sh scripts/uat.sh batua-be/scripts/`
- [ ] `cp docker-compose.yml batua-be/`
- [ ] `cp .env .env.docker .env.example batua-be/`
- [ ] `cp Backend_Dev_Rules.md batua-be/`
- [ ] `cp docs/api-*.md docs/storefront-home-api.md batua-be/docs/`
- [ ] `cp -R plans/batua-be-port batua-be/plans/`

## Phase 3 — Adapt

- [ ] Adapted `Makefile` written (no frontend `dev`/`stop` lines)
- [ ] Adapted `README.md` written (backend-only)
- [ ] Adapted `CLAUDE.md` written (backend + shared sections only)
- [ ] Adapted `.gitignore` written (no `site/` lines)

## Phase 4 — Verify byte parity

- [ ] `diff -rq batua/src batua-be/src` → empty
- [ ] `diff -rq batua/migrations batua-be/migrations` → empty
- [ ] `diff -rq batua/tests batua-be/tests` → empty (excluding UAT report dirs not copied)
- [ ] `cmp batua/Cargo.toml batua-be/Cargo.toml` exits 0
- [ ] `cmp batua/Cargo.lock batua-be/Cargo.lock` exits 0

## Phase 5 — Build & test parity

- [ ] `cd batua-be && cargo check` exits 0
- [ ] `cd batua-be && cargo test --lib --bins` matches the source repo's pass count
- [ ] `cd batua-be && cargo test --tests` matches source-repo behavior (or document divergences)
- [ ] `tests/lint_tracing.rs` passes

## Phase 6 — Database parity (see 10-database.md)

- [ ] All 30 migrations apply to a fresh `batua_be_verify` DB
- [ ] Spot-check shows expected tables
- [ ] Verification DB dropped

## Phase 7 — Smoke run

- [ ] `cargo run` from `batua-be/` boots the server (use `PORT=3001` to avoid clashing with the source repo)
- [ ] Server registers routes without panic (check log output)
- [ ] Server shuts down cleanly

## Phase 8 — Report

- [ ] Summary delivered to user: file counts, diff results, build/test pass count, migration check, smoke run outcome
