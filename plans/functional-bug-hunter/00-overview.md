# Functional Bug Hunter — Batua Test Suite

## Goal

Apply the Functional Bug Hunter methodology to Batua's untested business logic layers. Generate behavioral tests that probe for real bugs in the helpers.rs files across all services — focusing on boundary values, state transitions, arithmetic correctness, and constraint validation.

## Current State

### Already Well-Tested (skip these)
- **Ledger layer**: 30+ edge case tests in `tests/ledger_edge_cases.rs` — overdraw, zero amounts, negative amounts, precision, expiry, immutability, idempotency, pagination, bucket isolation, concurrent writes
- **Integration tests**: 20+ tests in `tests/integration_tests.rs` — wallet scoping, balance calculations, COD lifecycle, earn flow, redemption balance reduction, rule evaluation with conditions
- **Rules helpers**: 20+ unit tests in `src/services/rules/helpers.rs` — condition operators, reward calculation, campaign multipliers

### Untested (our targets)
1. **Redemption helpers** — `evaluate_bucket_eligibility`, `validate_constraints` (pure functions, no DB needed)
2. **Gift card helpers** — `generate_gift_card_code`, `issue`, `claim`, `redeem` (integration tests needed)
3. **Earn helpers** — `parse_bucket_type`, `extract_customer_phone/email/name`, `generate_earn_idempotency_key` (pure), plus `process_earn`, milestones, streaks, spin wheel (integration)
4. **Loyalty helpers** — `find_qualifying_tier`, `evaluate_tier`, `get_earn_multiplier` (integration)

## Scope

### In Scope
- Unit tests for pure functions (no DB) in `rules/helpers.rs` gaps, `redemption/helpers.rs`, `earn/helpers.rs`
- Integration tests for gift card lifecycle, earn flow edge cases, loyalty tier evaluation, spin wheel
- State machine transition tests for RedemptionState, CodOrderState

### Out of Scope
- Frontend E2E tests (separate effort)
- Screenshot evidence generation (not applicable to Rust backend tests)
- Performance/load testing
- API handler-level HTTP tests (focus on helpers layer)

## Success Criteria
- Every pure function in helpers.rs files has unit tests covering happy path + edge cases
- Every state machine has tests for all legal and illegal transitions
- Arithmetic edge cases (boundary values, rounding, zero, negative) are covered
- All tests pass with `cargo test`
- No false positives — tests verify real behavior, not implementation details

## Dependencies
- Test database available (`batua_test` or `TEST_DATABASE_URL`)
- Existing test infrastructure in `tests/integration_tests.rs` (reuse helpers)
