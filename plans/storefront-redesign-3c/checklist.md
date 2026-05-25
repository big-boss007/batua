# Implementation Checklist

## Phase 3: Type Definitions
- [ ] Verify CustomerIdentity.name is populated by lookupCustomer
- [ ] Define DateGroup type locally

## Phase 4: State Management
- [ ] Capture customer.name in page state
- [ ] Add $derived for date-grouped entries
- [ ] Add $derived for running balances

## Phase 6: Utilities
- [ ] Implement groupEntriesByDate
- [ ] Implement computeRunningBalances
- [ ] Implement getInitials
- [ ] Implement formatDateLabel

## Phase 7: UI Components
- [ ] Rework MerchantHeader — add avatar with initials
- [ ] Create ProfileBar component
- [ ] Rework BalanceCard → center-aligned hero (no gradient card)
- [ ] Create StatGrid component (2x2 grid)
- [ ] Rework TierCard → segmented progress bar
- [ ] Rework TransactionList → date-grouped
- [ ] Rework TransactionCard → vertical color bar + running balance
- [ ] Update ui/index.ts barrel

## Phase 8: Integration
- [ ] Wire new components into +page.svelte
- [ ] Replace card layout with divider layout
- [ ] Remove phone bar, add Switch to header
- [ ] Verify loading/error/empty states
- [ ] Test dark theme rendering

## Verification
- [ ] Customer name displays in profile bar
- [ ] Avatar shows initials
- [ ] Balance is large, centered
- [ ] 2x2 stat grid renders correctly
- [ ] Tier shows segmented progress
- [ ] Transactions grouped by date
- [ ] Running balance shows per transaction
- [ ] No regressions on phone input flow
- [ ] Dark theme works
