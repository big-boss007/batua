# Phase 5: Implementation — Integration Tests

## Objective
Write integration tests in `tests/functional_tests.rs` for gift card, loyalty, earn, redemption, and spin wheel flows.

## File: tests/functional_tests.rs

### Structure
- Reuse test infrastructure pattern from `tests/integration_tests.rs`
- `get_test_pool()`, `create_test_merchant()`, `create_test_customer()`, `create_test_wallet()`
- Each test is independent (creates its own merchant/customer/wallet)

### Gift Card Tests (GC-001 through GC-013)
- Requires: gift_cards::helpers, wallet_storage, ledger_storage
- Creates gift cards, claims, redeems, checks balances

### Loyalty Tests (LT-001 through LT-010)
- Requires: loyalty::helpers, loyalty::storage, ledger_storage
- Creates programs, tiers, credits ledger entries, evaluates tiers

### Earn Flow Tests (EF-001 through EF-007)
- Requires: earn::helpers, events::storage, rules::storage, ledger_storage
- Creates events, rules, processes earn, checks entries

### Redemption State Machine Tests (RSM-001 through RSM-012)
- Requires: redemption::helpers, redemption::storage
- Creates redemptions, attempts state transitions

### Spin Wheel Tests (SW-001 through SW-006)
- Requires: earn::helpers, earn::storage
- Creates wheel configs, segments, spins
