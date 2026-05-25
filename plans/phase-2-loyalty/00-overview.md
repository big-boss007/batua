# Phase 2: Loyalty Programs — Overview

**Status:** COMPLETED

## Goal

Build a loyalty tier system that lets merchants create tiered loyalty programs, automatically evaluate customers against tier thresholds, and provide tier-aware earn-rate multipliers for the wallet/ledger system.

## Scope

### Backend (`src/services/loyalty/`)
- CRUD for loyalty programs (one per merchant)
- CRUD for loyalty tiers (ranked, with thresholds and earn multipliers)
- Customer tier tracking with qualifying value history
- Tier evaluation engine supporting spend, points, and order_count criteria
- Bulk evaluation of all merchant customers
- Tier distribution analytics
- Earn-rate multiplier lookup for integration with the ledger earn flow

### Frontend (`frontend/src/lib/client/modules/customers/`)
- Loyalty program creation/editing form
- Tier creation form with benefits JSON editor
- Tier badge and tier progress bar components
- Customer detail view showing tier info
- Tier distribution chart for merchant analytics
- Customer search component

### Database (`20260318000012_loyalty_programs.sql`)
- `loyalty_programs` table (one per merchant via UNIQUE constraint)
- `loyalty_tiers` table (ranked tiers per program, UNIQUE on program_id + rank)
- `customer_tiers` table (one active tier per customer-merchant pair)

## Success Criteria

- [x] Merchants can create a loyalty program with evaluation criteria
- [x] Tiers can be defined with thresholds and earn-rate multipliers
- [x] Customer tiers are evaluated based on spend, points, or order count
- [x] Bulk evaluation runs across all merchant customers
- [x] Tier distribution analytics available per merchant
- [x] Earn-rate multiplier integrates with ledger earn flow
- [x] Frontend provides full CRUD for programs and tiers
- [x] Tier progress shown on customer detail view

## Dependencies

- Phase 0: Foundation (AppState, error handling, shared middleware)
- Phase 1: Wallets & Ledger (ledger_entries for qualifying value calculations, wallets for customer lookup)
- Identity service (customer_order_stats for order_count criteria)
