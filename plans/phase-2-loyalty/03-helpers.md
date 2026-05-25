# Phase 2: Loyalty — Helpers

**Status:** COMPLETED

## Helper Functions (`src/services/loyalty/helpers.rs`)

All public async functions instrumented with `#[tracing::instrument(skip(pool), err(Debug))]`.

### Qualifying Value Calculators

| Function | Criteria | Data Source |
|----------|----------|-------------|
| `get_qualifying_value_spend` | `spend` | SUM of `currency_equivalent` from ledger_entries (earned_credit, movement_type=in), optionally within evaluation_period_days |
| `get_qualifying_value_points` | `points` | SUM of `earning_unit` from ledger_entries (movement_type=in), optionally within evaluation_period_days |
| `get_qualifying_value_order_count` | `order_count` | `total_orders` from `customer_order_stats` table |

### Core Logic

| Function | Purpose |
|----------|---------|
| `find_qualifying_tier` | Pure function: iterates tiers in reverse rank order, returns first tier where `qualifying_value >= threshold` |
| `evaluate_tier` | Full evaluation: fetches program/tiers, computes qualifying value, determines tier change (upgrade/downgrade/none), upserts if changed |
| `get_customer_tier_info` | Assembles `CustomerTierResponse` with tier, program, and progress-to-next-tier calculation |
| `get_earn_multiplier` | Returns the earn_rate_multiplier for a customer's current tier (defaults to 1.0 if no program/tier) |

### Evaluation Flow

1. Load program for merchant (error if not found)
2. Load tiers (return no-change if empty)
3. Compute qualifying value based on `evaluation_criteria`
4. Find qualifying tier via `find_qualifying_tier`
5. Compare with current tier to determine change direction
6. Upsert new tier if changed
7. Return `TierEvaluationResult`
