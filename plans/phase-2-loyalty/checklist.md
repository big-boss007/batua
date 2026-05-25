# Phase 2: Loyalty — Checklist

**Status:** COMPLETED

## Backend

- [x] Define types in `src/services/loyalty/types.rs`
  - [x] `LoyaltyProgram` with sqlx::FromRow
  - [x] `LoyaltyTier` with threshold and earn_rate_multiplier
  - [x] `CustomerTier` with qualifying_value tracking
  - [x] `CreateProgramRequest` and `CreateTierRequest`
  - [x] `TierEvaluationResult` and `CustomerTierResponse`
  - [x] `TierProgress` for next-tier progress

- [x] Implement storage in `src/services/loyalty/storage.rs`
  - [x] `create_program` with unique violation handling
  - [x] `get_program` and `get_program_by_id`
  - [x] `update_program`
  - [x] `create_tier` with unique violation on (program_id, rank)
  - [x] `get_tiers` ordered by rank
  - [x] `get_tier_by_id`
  - [x] `get_customer_tier`
  - [x] `upsert_customer_tier` with ON CONFLICT
  - [x] `get_tier_distribution` with LEFT JOIN aggregation

- [x] Implement helpers in `src/services/loyalty/helpers.rs`
  - [x] `get_qualifying_value_spend` with optional period
  - [x] `get_qualifying_value_points` with optional period
  - [x] `get_qualifying_value_order_count`
  - [x] `find_qualifying_tier` pure function
  - [x] `evaluate_tier` full evaluation with upsert
  - [x] `get_customer_tier_info` with progress calculation
  - [x] `get_earn_multiplier` for ledger integration

- [x] Implement handlers in `src/services/loyalty/handler.rs`
  - [x] `create_program` POST handler
  - [x] `get_program` GET handler with db_reader
  - [x] `create_tier` POST handler
  - [x] `get_tiers` GET handler with db_reader
  - [x] `get_customer_tier_info` GET handler with db_reader
  - [x] `evaluate_tier` POST handler
  - [x] `evaluate_all_tiers` POST handler
  - [x] `get_tier_distribution` GET handler with db_reader

- [x] Configure router in `src/services/loyalty/mod.rs`
- [x] All functions have `#[tracing::instrument]`

## Database

- [x] Migration `20260318000012_loyalty_programs.sql`
  - [x] `loyalty_programs` table with UNIQUE merchant_id
  - [x] `loyalty_tiers` table with UNIQUE (program_id, rank)
  - [x] `customer_tiers` table with UNIQUE (customer_id, merchant_id)
  - [x] Indexes on foreign keys

## Frontend

- [x] Types defined in `customers/types.ts`
- [x] API calls in `customers/remote.ts`
  - [x] `searchCustomers`, `getCustomerDetail`
  - [x] `fetchLoyaltyProgram`, `fetchTiers`, `fetchTierDistribution`
  - [x] `createProgram`, `createTier`, `evaluateTier`
  - [x] `fetchMerchantCustomers`
- [x] Stores in `customers/store.ts`
  - [x] `customerSearchStore`
  - [x] `customerDetailStore`
  - [x] `loyaltyStore`
- [x] Utils in `customers/utils.ts`
  - [x] `getTierColor`, `formatMultiplier`, `sortTiersByRank`
  - [x] `formatMovementType`, `formatBucketType`
- [x] Barrel exports in `customers/index.ts`
- [x] UI components
  - [x] `LoyaltyProgramForm` with criteria selector
  - [x] `TierForm` with benefits JSON editor
  - [x] `TierBadge` using Pill component
  - [x] `TierProgress` using Progress component
  - [x] `TierDistributionChart` with horizontal bars
  - [x] `CustomerDetail` with wallet, tier, transactions
  - [x] `CustomerSearch` with debounced search
