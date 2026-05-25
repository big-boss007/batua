# Dual Unit Display — Points + Money

## Goal

Switch the entire Batua display layer from "everything in ₹" to a dual-unit system where:
- **Points buckets** (earned_credit, cod_pending, referral_reward, goodwill_credit, membership_benefit) display in merchant-branded points (e.g., "1,400 Stars")
- **Money buckets** (gift_card, customer_funded, refund_credit) display in ₹
- Both convert to a unified ₹ total at checkout/redemption

## Scope

### In Scope
- DB migration: add `points_name`, `points_icon`, `points_to_currency_rate` to merchants
- Backend: balance endpoint returns dual-unit breakdown (points vs cash), expiring-soon data
- Backend: merchant API includes points config in response
- Frontend storefront: Design 5 (stacked sections) with all 8 states per `docs/storefront-design5-states.html`
- Frontend admin: 8 areas per `docs/admin-designs-dual-unit.html` (dashboard, transactions, customer detail, analytics, settings, wallet policy, referral form, customer transactions)
- Frontend foundation: new `formatPoints()` utility, bucket classification helper

### Out of Scope
- Gift card pages (stay ₹, no change)
- Gift card issue/bulk forms (stay ₹)
- Redemption history (stay ₹ — checkout currency)
- Platform/super-admin pages (metadata only)
- Changing how the ledger stores data (earning_unit/currency_equivalent stay as-is)
- Changing redemption flow logic (already converts to ₹ via rate)

## Success Criteria
- Storefront shows points as primary unit with ₹ equivalent
- Gift cards display in ₹ everywhere, never converted to points
- Admin sees dual-unit metrics, bucket-tagged breakdowns
- Merchant can configure points_name, icon, and rate from settings
- `cargo test` and `npx svelte-check` pass
- All 8 storefront states render correctly
- Existing conversion_rate=1.0 merchants see "1 Star = ₹1" (backward compatible)

## Dependencies
- Design mockups finalized: `docs/storefront-design5-states.html`, `docs/admin-designs-dual-unit.html`
- Existing seed data in DB for visual testing

## Implementation Order
1. Database migration (new merchant columns)
2. Backend types + storage changes
3. Backend handler changes (balance endpoint, merchant endpoint)
4. Frontend foundation (formatPoints, bucket classification)
5. Frontend storefront (Design 5 rebuild)
6. Frontend admin (8 areas)
7. Seed data update
8. Verification
