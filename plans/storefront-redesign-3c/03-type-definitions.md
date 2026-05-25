# Phase 3: Type Definitions

## Objective

Ensure types support the new data needs for 3C.

## Analysis

### Existing types that cover our needs:
- `CustomerIdentity` — has `name: string | null` (for profile bar)
- `CustomerBalance` — has `buckets` array (for stat grid)
- `BucketBalance` — has `bucket_type`, `displayed`, `spendable`
- `CustomerTierInfo` — has `tier_name`, `progress_to_next`
- `TransactionEntry` — has `created_at`, `currency_equivalent`, `movement_type`

### New type needed in `+page.svelte` (local, not in types.ts):
- `DateGroup` — `{ date: string; label: string; entries: TransactionEntry[] }` for date-grouped transactions

### Data to derive client-side:
- **Lifetime saved**: Sum of all "In" movement transactions (approximation from visible entries, or show only if available)
- **Expiring soon**: Filter buckets or entries for near-expiry amounts (if data available; otherwise omit the stat)
- **Running balance**: Cumulative sum walking transactions in chronological order

## Tasks

- [ ] No changes to `types.ts` needed — all existing types suffice
- [ ] Define `DateGroup` type locally in the page or as a helper type in utils
