# Phase 3: Helpers / Business Logic

## Objective
No major business logic changes needed. The dual-unit display is primarily a presentation concern. Document what stays the same and minor adjustments.

## What stays the same
- `earn/helpers.rs`: `process_earn` already writes both `earning_unit` and `currency_equivalent` correctly
- `redemption/helpers.rs`: `evaluate_eligibility` operates on `spendable` (earning_unit) — this is correct, as redemption converts to ₹ at the boundary
- `rules/helpers.rs`: `calculate_reward` already computes both earning_unit and currency_equivalent
- `gift_cards/helpers.rs`: gift cards set earning_unit = currency_equivalent = amount, conversion_rate = 1.0 — correct for cash buckets

## Minor changes

### `src/services/ledger/types.rs` — `BucketType::is_points_bucket()`
Add the method to classify bucket types (listed in Phase 1).

### `src/services/ledger/storage.rs` — `build_wallet_balance`
Classify buckets when building the response, sum points vs cash separately (listed in Phase 2).

## Validation
- `cargo test` — all 211 existing tests pass
- No business logic changes means no new edge cases
