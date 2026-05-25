# Storefront Redesign — Concept 3C: Summary Dashboard

## Goal

Redesign the storefront loyalty hub (`/s/[slug]`) to a clean, minimal, Apple Wallet-inspired design with:
- Customer name + profile bar with lifetime stats
- Centered hero balance
- 2x2 stat grid (Earned, COD Pending, On Hold, Expiring Soon)
- Segmented tier progress bar
- Date-grouped transactions with running balance
- Thin vertical color-bar indicators instead of icon circles

## Scope

### In Scope
- Rework `+page.svelte` layout and information hierarchy
- Rework `BalanceCard.svelte` → new hero balance + stat grid
- Rework `TierCard.svelte` → segmented progress bar
- Rework `TransactionList.svelte` + `TransactionCard.svelte` → date-grouped list with running balance and color bars
- Rework `MerchantHeader.svelte` → add avatar with initials
- Add customer name display (from `CustomerIdentity.name`)
- Add "Expiring Soon" stat (new data from API or derived)
- Add lifetime savings stat (new data from API or derived)
- Add running balance calculation for transactions

### Out of Scope
- Backend API changes (use existing data; derive new metrics client-side where possible)
- PhoneInput redesign (keep as-is)
- ReferralCard redesign
- GiftCardStatus redesign
- New API endpoints

## Information Hierarchy (New)

1. **Identity + Lifetime Value** — Profile bar: name, tier, order count, member-since, lifetime saved
2. **Current Balance** — Large centered hero number + pending callout
3. **Bucket Breakdown** — 2x2 stat grid with color-coded cards
4. **Tier Progress** — Segmented bar with benefit callout
5. **Transaction History** — Date-grouped with running balance

## Success Criteria

- [ ] Customer name displayed in profile bar
- [ ] Avatar with initials in header
- [ ] Hero balance is center-aligned, 48px+
- [ ] 2x2 stat grid replaces inline bucket bars
- [ ] Segmented tier progress (6 segments)
- [ ] Transactions grouped by date
- [ ] Running balance shown per transaction
- [ ] Horizontal dividers instead of card borders
- [ ] Dark theme works correctly
- [ ] No regressions on phone input / loading / error states

## Dependencies

- `CustomerIdentity` type already has `name` field
- Existing API provides all balance/tier/transaction data
- Lifetime savings + expiring soon may need to be derived client-side or stubbed
