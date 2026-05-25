# Phase 2: Loyalty — Integration

**Status:** COMPLETED

## Backend Integration

### Inbound Dependencies (what loyalty uses)
- **Ledger service**: Queries `ledger_entries` joined with `wallets` to compute qualifying values for spend and points criteria
- **Wallets service**: Queries distinct `customer_id` from `wallets` for bulk evaluation
- **Identity service**: Uses `customer_order_stats` table for order_count criteria

### Outbound Dependencies (what uses loyalty)
- **Ledger earn flow**: Calls `helpers::get_earn_multiplier(pool, merchant_id, customer_id)` to apply tier-based earn-rate multipliers when crediting points

## Frontend Integration

### Module: `customers/`
- **remote.ts**: API calls for loyalty programs, tiers, distribution, evaluation
- **store.ts**: Three stores - `customerSearchStore`, `customerDetailStore`, `loyaltyStore`
- **utils.ts**: Tier color mapping, multiplier formatting, tier sorting, movement/bucket type formatting

### UI Components
| Component | Purpose |
|-----------|---------|
| `LoyaltyProgramForm` | Create/edit program with name and evaluation criteria selector |
| `TierForm` | Create/edit tier with rank, threshold, multiplier, JSON benefits editor |
| `TierBadge` | Pill showing tier name and multiplier |
| `TierProgress` | Progress bar toward next tier |
| `TierDistributionChart` | Horizontal bar chart of customer distribution across tiers |
| `CustomerDetail` | Full customer view with wallet, tier, and recent transactions |
| `CustomerSearch` | Debounced search by phone or external ID |

### Component Library Usage
- `Pill` from `@juspay/svelte-ui-components` for tier badges and status
- `Progress` for tier progress and distribution bars
- `Input`, `Button`, `Select` for forms
- `Avatar` for customer identity display
- `Table` referenced for data display
