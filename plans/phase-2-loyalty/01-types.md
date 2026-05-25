# Phase 2: Loyalty — Types

**Status:** COMPLETED

## Backend Types (`src/services/loyalty/types.rs`)

### Domain Models

| Type | Derives | Purpose |
|------|---------|---------|
| `LoyaltyProgram` | `Debug, Clone, Serialize, sqlx::FromRow` | Program config per merchant |
| `LoyaltyTier` | `Debug, Clone, Serialize, sqlx::FromRow` | Individual tier with threshold and multiplier |
| `CustomerTier` | `Debug, Clone, Serialize, sqlx::FromRow` | Tracks which tier a customer is in |

### Request Types

| Type | Purpose |
|------|---------|
| `CreateProgramRequest` | `merchant_id`, `name`, `evaluation_criteria`, `evaluation_period_days` |
| `CreateTierRequest` | `program_id`, `name`, `rank`, `threshold`, `earn_rate_multiplier`, `benefits` (JSON) |

### Response Types

| Type | Purpose |
|------|---------|
| `TierEvaluationResult` | Result of evaluating a customer: `current_tier`, `new_tier`, `changed`, `direction` |
| `CustomerTierResponse` | Full tier info for a customer: tier, program, progress_to_next |
| `TierProgress` | Next-tier progress: `next_tier_name`, `current_value`, `threshold`, `percentage` |

### Helper-Local Types

| Type | Location | Purpose |
|------|----------|---------|
| `QualifyingValueRow` | `helpers.rs` | Single `value: f64` row for qualifying value queries |

### Storage-Local Types

| Type | Location | Purpose |
|------|----------|---------|
| `TierDistributionRow` | `storage.rs` | `tier_name`, `customer_count` for distribution aggregation |

## Frontend Types (`frontend/src/lib/client/modules/customers/types.ts`)

| Type | Purpose |
|------|---------|
| `Customer` | Core customer record |
| `CustomerDetail` | Composite: customer + wallet + tier + recent_entries |
| `WalletSummary` | Wallet balance snapshot |
| `CustomerTierInfo` | Tier name, rank, multiplier, progress |
| `TierProgress` | Progress toward next tier |
| `LedgerEntrySummary` | Abbreviated ledger entry for transaction list |
| `LoyaltyProgram` | Program summary (id, name, criteria, active) |
| `LoyaltyTier` | Tier config (id, name, rank, threshold, multiplier, benefits) |
| `TierDistribution` | Tier name + customer count |
| `MerchantCustomerRow` | Customer row for merchant customer list |
