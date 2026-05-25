# Phase 9: Frontend Integration — Storefront + Admin

## Objective
Implement all UI changes across storefront and admin. This is the largest phase.

## Part A: Foundation Layer (2 files)

### `frontend/src/lib/client/modules/foundation/utils.ts`
```typescript
function formatPoints(value: number, icon: string): string {
  return `${value.toLocaleString('en-IN')}${icon ? ' ' + icon : ''}`;
}
```

### `frontend/src/lib/client/modules/foundation/index.ts`
Export `formatPoints`

### Bucket classification helper (in foundation or storefront utils)
```typescript
const POINTS_BUCKETS = new Set([
  'earned_credit', 'cod_pending', 'referral_reward',
  'goodwill_credit', 'membership_benefit'
]);
const CASH_BUCKETS = new Set(['gift_card', 'customer_funded', 'refund_credit']);

function isPointsBucket(bucketType: string): boolean {
  return POINTS_BUCKETS.has(bucketType);
}
```

## Part B: Storefront (10-12 files) — Design 5 Implementation

Reference: `docs/storefront-design5-states.html`

### `storefront/ui/BalanceCard.svelte` — FULL REWRITE
Replace ₹ hero with Design 5 stacked layout:
1. Total Redeemable (₹) at top
2. Stars card (green) — points_balance with ₹ equivalent, inline breakdown, expiry warning
3. Cash card (purple) — only when cash_balance > 0

Props change: needs `balance`, `merchant` (for points config), `expiringSoon`

Conditional rendering rules (from design states doc):
- Stars card: always show when logged in (show 0★ with CTA if empty)
- Stars card color: green (spendable) / yellow (all pending) / red-tint (expiring) / gray (0★)
- Cash card: only when cash_balance > 0
- Expiry line: only when expiringSoon exists and days <= 30
- Breakdown line: only when multiple source buckets exist

### `storefront/ui/StatGrid.svelte` — REMOVE
Replaced by inline breakdown inside the Stars card. Delete this component.

### `storefront/ui/TransactionCard.svelte`
- Check `isPointsBucket(entry.bucket_type)`:
  - Points: show `formatPoints(entry.earning_unit, merchant.points_icon)` with "≈ ₹X" hint
  - Cash: show `formatCurrencyINR(entry.currency_equivalent)` with "cash" hint
- Running balance: show in native unit of the transaction

### `storefront/ui/TransactionList.svelte`
- Pass merchant config through to TransactionCard
- Running balance computation needs to stay in ₹ (since it's a combined wallet balance)

### `storefront/ui/TierCard.svelte`
- Replace "points" text with merchant `points_icon`
- "12,000 points" → "12,000 ★"

### `storefront/ui/ReferralCard.svelte`
- `formatCurrencyINR(referralReward)` → `formatPoints(referralReward * (1/rate), icon)`
- "earn ₹100 for each referral" → "earn 400 ★ for each referral"
- Share text updated similarly

### `storefront/ui/GiftCardStatus.svelte` — NO CHANGE
Gift cards stay ₹.

### `storefront/utils.ts`
- `computeRunningBalances`: keep using currency_equivalent (combined ₹ balance for running total)

### `storefront/types.ts`
- Add new fields per Phase 1

### `routes/s/[slug]/+page.svelte`
- Pass merchant config to BalanceCard, TransactionCard, TierCard, ReferralCard
- Remove StatGrid usage
- Lifetime saved stays in ₹
- Empty state: use merchant points_name in copy

### `routes/s/[slug]/+layout.svelte`
- Ensure merchant data includes points config fields

### `routes/s/[slug]/refer/[code]/+page.svelte`
- Referral reward display: ₹ → ★

### `storefront/ui/index.ts`
- Remove StatGrid export, keep everything else

## Part C: Admin (15 files)

Reference: `docs/admin-designs-dual-unit.html`

### C1. Dashboard — `routes/admin/+page.svelte`
- MetricCards for Active Credits, Total Earned, Total Redeemed, COD Pending:
  - `value` becomes points formatted: "1,94,000 ★"
  - Add subtitle with ₹ equivalent
- Need merchant points config (already in currentMerchant store)

### C2. Analytics — `analytics/ui/OverviewCards.svelte`
- Same pattern: ★ primary, ₹ subtitle for credits/redeemed/expired
- "Active Credits" label → "Active Stars" (use merchant points_name)

### C3. Analytics — `analytics/ui/CodMetrics.svelte`
- COD amounts: ★ with ₹ hint

### C4. Analytics — `analytics/ui/CampaignPerformanceTable.svelte`
- `total_value`, `average_reward`: ★ primary + ₹ hint

### C5. Transactions — `transactions/ui/TransactionTable.svelte`
- Column headers: "Amount" → "Points / Cash", "Currency Equiv." → "₹ Value"
- Cell rendering: check `isPointsBucket` → format as ★ or ₹
- Add movement prefix (+/-) to points/cash column

### C6. Transactions — `transactions/ui/BalanceCard.svelte`
- 3-column layout: Stars Balance / Cash Balance / Total ₹ Value
- Bucket rows: tag each as POINTS or CASH, show native unit

### C7. Customers — `customers/ui/CustomerDetail.svelte`
- Wallet card: 3-column (Stars / Cash / Total)
- Bucket breakdown: tagged POINTS/CASH
- Recent transactions: native unit display (same as storefront pattern)

### C8. Transactions — `transactions/ui/RedemptionHistory.svelte` — NO CHANGE
Redemptions are ₹.

### C9. Settings — `routes/admin/settings/+page.svelte`
- Add new "Points Configuration" section (form with points_name, points_icon, rate, preview)

### C10. Settings — `settings/ui/WalletPolicyForm.svelte`
- Tag bucket as POINTS or CASH
- Points buckets: labels use ★ with ₹ conversion hint
- Cash buckets: labels stay ₹
- "Max Per Order Fixed" always ₹ (checkout currency)

### C11. Referrals — `referrals/ui/ReferralProgramForm.svelte`
- Labels: "(₹)" → "(★)" with ₹ conversion hint

### C12. Gift Cards — NO CHANGE
- `gift-cards/ui/GiftCardDetail.svelte` — stays ₹
- `gift-cards/ui/GiftCardsList.svelte` — stays ₹
- `gift-cards/ui/IssueGiftCardForm.svelte` — stays ₹
- `gift-cards/ui/BulkIssueForm.svelte` — stays ₹
- `gift-cards/ui/GiftCardConfirmation.svelte` — stays ₹

### C13. Admin Transactions Page — `routes/admin/transactions/+page.svelte`
- Detail panel: show native unit for amounts
- Update column definitions to match TransactionTable changes

## Validation
- `npx svelte-check --threshold error` passes
- Visual verification: storefront loads with all 8 states
- Visual verification: admin dashboard, transactions, customer detail show dual-unit
- Test with rate=1.0 (backward compatible) and rate=0.25 (4:1)
