# Referrals Redesign

## Goal
Redesign the referrals section into two sub-pages: "Referral Program" (setup, config, dashboard) and "Influencers" (creator codes with vanity codes and commissions). Replace tabs with single-page dashboard. Use mobile numbers instead of customer IDs. Use store's actual points currency.

## Scope
- Sidebar: Referrals becomes parent with two children
- `/admin/referrals` page: Complete rewrite (empty → setup → dashboard)
- `/admin/influencers` page: New page (empty → add modal → list)
- Points currency from merchant store (name + icon)
- Mobile numbers for customer identification
- Commission as percentage with input-with-suffix pattern

## Out of Scope
- Backend API changes
- Storefront referral flow
- Customer lookup by mobile (uses existing customer_id internally)

## Design Spec
`docs/referrals-redesign-v2.html` — 6 states approved by user

## Success Criteria
- All 6 design states render correctly
- `svelte-check` passes with 0 errors
- Points currency reflects store settings
