# Phase 2: Test Generation — Pure Function Unit Tests

## Objective
Generate unit tests for all pure functions (no DB dependency) that currently lack coverage.

## Test File: src/services/redemption/helpers.rs (add #[cfg(test)] mod)

### evaluate_bucket_eligibility tests
| Test ID | Input | Expected | Category |
|---------|-------|----------|----------|
| RBE-001 | spendable=0, any policy | 0.0 | boundary |
| RBE-002 | spendable=-10, any policy | 0.0 | negative |
| RBE-003 | spendable=500, no policy | 500.0 | no-policy |
| RBE-004 | spendable=500, policy.is_active=false | 0.0 | inactive |
| RBE-005 | spendable=500, excluded_pm=["upi"], pm="UPI" | 0.0 | case-insensitive |
| RBE-006 | spendable=500, excluded_pm=["upi"], pm="card" | >0 | non-excluded |
| RBE-007 | spendable=500, max_pct=10, order=1000 | 100.0 | pct cap |
| RBE-008 | spendable=500, max_fixed=50 | 50.0 | fixed cap |
| RBE-009 | spendable=500, max_pct=10, max_fixed=200, order=1000 | 100.0 | both caps, pct wins |
| RBE-010 | spendable=500, max_pct=50, max_fixed=200, order=1000 | 200.0 | both caps, fixed wins |
| RBE-011 | spendable=50, max_pct=100, order=1000 | 50.0 | spendable < cap |
| RBE-012 | spendable=500, no pm in context | no exclusion | null pm |

### validate_constraints tests
| Test ID | Input | Expected | Category |
|---------|-------|----------|----------|
| RVC-001 | requested=0 | error: must be positive | boundary |
| RVC-002 | requested=-10 | error: must be positive | negative |
| RVC-003 | requested=200, eligible=100 | error: exceeds eligible | overflow |
| RVC-004 | requested=100, eligible=100 | ok(100) | exact boundary |
| RVC-005 | discount_codes=["X"], stackable=false | error: cannot stack | stackability |
| RVC-006 | discount_codes=["X"], stackable=true | ok | stackability |
| RVC-007 | amount=5, min_redemption=10 | error: below min | min threshold |
| RVC-008 | amount=10, min_redemption=10 | ok(10) | exact min boundary |
| RVC-009 | amount=75, step_size=50 | ok(50) | step snap down |
| RVC-010 | amount=100, step_size=50 | ok(100) | exact step multiple |
| RVC-011 | amount=25, step_size=50 | error: zero after snap | step eliminates |
| RVC-012 | amount=0.01, step_size=1.0 | error: zero after snap | small amount step |
| RVC-013 | discount_codes=[], stackable=false | ok | empty codes = no check |
| RVC-014 | amount=100, no policies | ok(100) | no constraints |

## Test File: src/services/earn/helpers.rs (add unit tests)

### parse_bucket_type tests
| Test ID | Input | Expected |
|---------|-------|----------|
| PBT-001 | "earned_credit" | Ok(EarnedCredit) |
| PBT-002 | "EarnedCredit" | Ok(EarnedCredit) |
| PBT-003 | "unknown" | Err |
| PBT-004 | "" | Err |
| PBT-005 | all 8 snake_case variants | Ok |
| PBT-006 | all 8 PascalCase variants | Ok |

### extract_customer_phone tests
| Test ID | Input | Expected |
|---------|-------|----------|
| ECP-001 | customer.phone="1234" | Ok("1234") |
| ECP-002 | customer.phone=None, order.phone="5678" | Ok("5678") |
| ECP-003 | customer.phone="", order.phone="5678" | Ok("5678") |
| ECP-004 | no phone anywhere | Err |
| ECP-005 | customer.phone="" , order.phone="" | Err |

### generate_earn_idempotency_key tests
| Test ID | Input | Expected |
|---------|-------|----------|
| GEK-001 | event_id=X, rule_snapshot=Some(Y) | "earn:X:Y" |
| GEK-002 | event_id=X, rule_snapshot=None | "earn:X" |

## Test File: src/services/loyalty/helpers.rs (add unit tests)

### find_qualifying_tier tests
| Test ID | Input | Expected |
|---------|-------|----------|
| FQT-001 | tiers=[100,500,1000], value=750 | tier at 500 |
| FQT-002 | tiers=[100,500,1000], value=1000 | tier at 1000 (exact boundary) |
| FQT-003 | tiers=[100,500,1000], value=50 | None |
| FQT-004 | tiers=[100,500,1000], value=100 | tier at 100 (exact boundary) |
| FQT-005 | tiers=[], value=500 | None |
| FQT-006 | tiers=[100,500,1000], value=99.99 | None (just below) |

## Test File: src/services/gift_cards/helpers.rs (add unit tests)

### generate_gift_card_code tests
| Test ID | Check |
|---------|-------|
| GCC-001 | Starts with "BRZE-" |
| GCC-002 | Format: BRZE-XXXX-XXXX-XXXX (19 chars total) |
| GCC-003 | No ambiguous chars (I, O, 0, 1) |
| GCC-004 | 100 generated codes are all unique |
