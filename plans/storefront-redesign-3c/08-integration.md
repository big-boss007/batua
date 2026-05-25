# Phase 8: Integration

## Objective

Wire everything together in `+page.svelte`.

## Changes to +page.svelte

### New state variables:
- `customerName: string | null = null` — from lookupCustomer result
- Derived: `dateGroupedEntries` — via groupEntriesByDate(entries)
- Derived: `runningBalances` — via computeRunningBalances(entries, balance.spendable_balance)

### Updated data fetch:
- Capture `customer.name` from lookupCustomer response (already returns CustomerIdentity)

### Updated success state layout:

```
MerchantHeader (with customerName for avatar)
ProfileBar (name, tier, lifetimeSaved)
─── divider ───
BalanceHero (spendable, total, pending)
─── divider ───
StatGrid (buckets)
─── divider ───
TierProgress (segmented)
─── divider ───
TransactionList (date-grouped, with running balances)
Bottom actions (Gift Card, Refer & Earn)
```

### Layout styling:
- Replace `.hub-section` card wrappers with simple divider `<hr>` elements
- Divider: 1px solid var(--color-border), full-bleed (negative margin to counteract padding)
- Remove gap from loyalty-hub container; use section padding instead

### Phone bar:
- Remove the separate phone bar ("Showing rewards for your account" + Change)
- "Switch" link moves to MerchantHeader

## Tasks

- [ ] Update +page.svelte state and data flow
- [ ] Wire new components into success state
- [ ] Replace card layout with divider layout
- [ ] Remove phone bar, move Switch to header
- [ ] Verify loading/error states still work
- [ ] Test dark theme
