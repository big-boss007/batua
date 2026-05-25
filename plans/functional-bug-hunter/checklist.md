# Functional Bug Hunter — Implementation Checklist

## Phase 1: Business Rule Extraction
- [x] Extract rules from redemption/helpers.rs
- [x] Extract rules from gift_cards/helpers.rs
- [x] Extract rules from earn/helpers.rs
- [x] Extract rules from loyalty/helpers.rs
- [x] Identify edge cases per rule

## Phase 2: Test Design
- [x] Design evaluate_bucket_eligibility tests (12 tests)
- [x] Design validate_constraints tests (14 tests)
- [x] Design parse_bucket_type tests (6 tests)
- [x] Design extract_customer_phone tests (5 tests)
- [x] Design find_qualifying_tier tests (6 tests)
- [x] Design generate_gift_card_code tests (4 tests)

## Phase 3: Integration Test Design
- [x] Design gift card lifecycle tests (13 tests)
- [x] Design loyalty tier evaluation tests (10 tests)
- [x] Design earn flow edge case tests (7 tests)
- [x] Design redemption state machine tests (12 tests)
- [x] Design spin wheel tests (6 tests)

## Phase 4: Implement Unit Tests
- [x] Add tests to src/services/redemption/helpers.rs (26 tests)
- [x] Add tests to src/services/earn/helpers.rs (16 tests)
- [x] Add tests to src/services/loyalty/helpers.rs (8 tests)
- [x] Add tests to src/services/gift_cards/helpers.rs (9 tests)

## Phase 5: Implement Integration Tests
- [x] Create tests/functional_tests.rs with test infrastructure
- [x] Implement gift card lifecycle tests (12 tests)
- [x] Implement loyalty tier tests (8 tests)
- [x] Implement redemption eligibility tests (4 tests)
- [x] Implement redemption state machine tests (5 tests)
- [x] Implement spin wheel tests (6 tests)
- [x] Implement earn flow E2E tests (4 tests)
- [x] Implement milestone tests (4 tests)
- [x] Implement streak tests (3 tests)
- [x] Implement membership/multiplier tests (4 tests)
- [x] Implement manual credit tests (2 tests)
- [x] Implement wallet policy-constrained eligibility tests (5 tests)

## Phase 9: Verification
- [x] All unit tests pass (91/91)
- [x] All integration tests pass (56/56 functional + 26/26 integration + 37/37 edge cases)
- [x] Total: 211 tests, 0 failures

## Bugs Discovered

### BUG-001: Streak achievement window_start uses exact timestamp match
- **File**: `src/services/earn/storage.rs:332-348`
- **Function**: `has_streak_achievement_in_window`
- **Severity**: P1 (data duplication, mitigated by ledger idempotency)
- **Category**: time/date boundary
- **Expected**: Second `check_and_award_streaks` call in same window should find prior achievement
- **Actual**: `WHERE window_start = $3` uses exact DateTime comparison, but `window_start = now() - days` changes every microsecond
- **Impact**: Duplicate `streak_achievements` rows created (ledger entry deduplicated via idempotency key, so no financial impact)
- **Fix**: Change `WHERE window_start = $3` to `WHERE window_start::date = $3::date` or use a range check
