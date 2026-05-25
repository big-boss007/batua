# Implementation Checklist

## Phase 1: Sidebar Navigation
- [ ] Update `+layout.svelte` — Referrals becomes parent with children: Referral Program, Influencers

## Phase 2: Referral Program Page
- [ ] Rewrite `/admin/referrals/+page.svelte` — 3 states: empty, setup, dashboard
- [ ] Empty state with setup hero + 3 steps
- [ ] Setup form with input-with-suffix fields (number + currency)
- [ ] Active dashboard: status bar, metrics, codes table, conversions
- [ ] Edit program mode
- [ ] Mobile numbers in tables

## Phase 3: Influencers Page
- [ ] Create `/admin/influencers/+page.svelte` — 3 states: empty, modal, list
- [ ] Empty state with CTA
- [ ] Add Influencer modal (mobile, vanity code, commission %)
- [ ] Influencers list with metrics cards and table

## Verification
- [ ] `npx svelte-check --threshold error` passes with 0 errors
- [ ] All 6 design states render correctly in browser
