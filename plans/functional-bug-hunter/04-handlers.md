# Phase 4: Implementation — Unit Tests

## Objective
Write the actual unit test code for pure functions identified in Phase 2.

## Files to Modify

### 1. `src/services/redemption/helpers.rs`
Add `#[cfg(test)] mod tests` block with:
- 12 tests for `evaluate_bucket_eligibility`
- 14 tests for `validate_constraints`

### 2. `src/services/earn/helpers.rs`
Add `#[cfg(test)] mod tests` block with:
- 6 tests for `parse_bucket_type`
- 5 tests for `extract_customer_phone`
- 2 tests for `generate_earn_idempotency_key`

### 3. `src/services/loyalty/helpers.rs`
Add `#[cfg(test)] mod tests` block with:
- 6 tests for `find_qualifying_tier`

### 4. `src/services/gift_cards/helpers.rs`
Add `#[cfg(test)] mod tests` block with:
- 4 tests for `generate_gift_card_code`

## Implementation Notes
- Use the same patterns as existing tests in `rules/helpers.rs`
- Pure function tests: no async, no PgPool, just construct inputs and assert outputs
- For `evaluate_bucket_eligibility`: construct WalletPolicy and OrderContext structs directly
- For `find_qualifying_tier`: construct LoyaltyTier vec with minimal fields
- For `extract_customer_phone`: construct ShopifyOrderPayload with targeted phone placements
