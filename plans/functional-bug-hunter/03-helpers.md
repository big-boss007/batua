# Phase 3: Test Generation — Integration Tests

## Objective
Generate integration tests for multi-step business logic that requires a database.

## Test File: tests/functional_tests.rs

### Gift Card Lifecycle Tests
| Test ID | Scenario | Expected |
|---------|----------|----------|
| GC-001 | Issue → claim → redeem full amount | All succeed, balance = 0 |
| GC-002 | Issue → claim → partial redeem → check remaining | Remaining = original - redeemed |
| GC-003 | Issue → claim → redeem more than balance | Error: insufficient balance |
| GC-004 | Issue → claim twice | Error: already claimed |
| GC-005 | Issue with amount=0 | Error: must be positive |
| GC-006 | Issue with amount=-100 | Error: must be positive |
| GC-007 | Claim expired gift card | Error: expired |
| GC-008 | Redeem inactive gift card | Error: not active |
| GC-009 | Bulk issue 5 cards → verify all unique codes | 5 distinct codes |
| GC-010 | Bulk issue idempotent replay | Same batch_id returns same cards |
| GC-011 | Redeem with amount=0 | Error: must be positive |
| GC-012 | Issue → redeem without claiming (bearer wallet) | Should work — redeem_wallet_id falls back to bearer |
| GC-013 | Claim → verify ledger has Across movement | Out from bearer, In to customer |

### Loyalty Tier Evaluation Tests
| Test ID | Scenario | Expected |
|---------|----------|----------|
| LT-001 | No program exists | Error: not found |
| LT-002 | Program exists, no tiers | No tier assigned |
| LT-003 | Spend-based: customer at 0 spend, tiers at [100,500] | No tier |
| LT-004 | Spend-based: customer at 150 spend, tiers at [100,500] | Tier at 100 |
| LT-005 | Spend-based: customer at exactly 500, tiers at [100,500] | Tier at 500 (boundary) |
| LT-006 | Tier upgrade: was at 100-tier, now qualifies for 500-tier | changed=true, direction=upgrade |
| LT-007 | Tier downgrade: was at 500-tier, evaluation period expired | changed=true, direction=downgrade |
| LT-008 | Same tier: re-evaluation produces same tier | changed=false |
| LT-009 | earn_multiplier: customer has tier with 1.5x multiplier | Returns 1.5 |
| LT-010 | earn_multiplier: no program → 1.0 | Default multiplier |

### Earn Flow Edge Cases
| Test ID | Scenario | Expected |
|---------|----------|----------|
| EF-001 | Event not in Received state | Error: wrong state |
| EF-002 | COD order → entry is Held in CodPending | movement=Held, bucket=CodPending |
| EF-003 | Prepaid order → entry is In in EarnedCredit | movement=In, bucket=EarnedCredit |
| EF-004 | No matching rules → 0 entries created | Empty entries_created |
| EF-005 | Loyalty multiplier 2x applied | earning_unit doubled |
| EF-006 | Membership multiplier 1.5x, loyalty 1.2x → max(1.5, 1.2) = 1.5x | 1.5x applied |
| EF-007 | Both multipliers 1.0 → no change | Original amounts preserved |

### Redemption State Machine Tests
| Test ID | From State | To State | Expected |
|---------|------------|----------|----------|
| RSM-001 | Initiated | Validating | Allowed |
| RSM-002 | Validating | Rejected | Allowed |
| RSM-003 | Validating | Committed | Allowed |
| RSM-004 | Committed | Applied | Allowed |
| RSM-005 | Applied | Completed | Allowed |
| RSM-006 | Applied | Failed | Allowed |
| RSM-007 | Failed | Compensated | Allowed |
| RSM-008 | Compensated | (terminal) | No further transitions |
| RSM-009 | Completed | (terminal) | No further transitions |
| RSM-010 | Rejected | Compensated | Error: invalid state for compensation |
| RSM-011 | Initiated | Compensated | Error: invalid state for compensation |
| RSM-012 | Completed | Compensated | Error: invalid state for compensation |

### Spin Wheel Tests
| Test ID | Scenario | Expected |
|---------|----------|----------|
| SW-001 | No wheel configured | Error: not found |
| SW-002 | Inactive wheel | Error: not active |
| SW-003 | Spin limit reached | Error: no spins remaining |
| SW-004 | All segments have reward > 0 | Ledger entry created |
| SW-005 | Winning segment reward = 0 | No ledger entry, still recorded |
| SW-006 | spins_remaining_today correct | limit - (spins_today + 1) |
