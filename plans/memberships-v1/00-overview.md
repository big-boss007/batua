# Memberships V1 — Merchant-Assigned with Earn Multiplier

## Goal

Launch memberships as a merchant-assigned feature where:
1. Merchants create membership plans (name, price, monthly/annual, earn multiplier)
2. Merchants manually assign customers to plans (1-year expiry)
3. The membership's earn_rate_multiplier is applied when cashback rules fire
4. The storefront shows the customer's membership status

## Architecture Decision

Memberships mirror loyalty tiers — both have an `earn_rate_multiplier`. The key difference:
- Loyalty tiers: permanent, earned by points accumulation
- Memberships: time-boxed (1 year), manually assigned by merchant

When calculating earn rate, use the **higher multiplier** between loyalty tier and membership. No stacking.

## Scope

### In Scope
- **Admin UI**: New "Memberships" sidebar nav item with plan CRUD, subscriber list, assign/cancel
- **Backend**: Wire membership + loyalty tier multiplier into `do_process_earn()` flow
- **Storefront**: Membership badge on profile bar (plan name + days remaining)
- **Admin sidebar**: Add "Memberships" nav item

### Out of Scope
- Customer self-serve purchase flow
- Payment integration (Razorpay/Shopify)
- Auto-renewal / recurring billing
- Benefits enforcement (free shipping, early access)
- Membership-specific storefront landing page
- Renewal reminders / expiry notifications

## Existing Backend

Already built in `src/services/earn/`:
- DB tables: `membership_plans`, `customer_memberships`
- Types: `MembershipPlan`, `CustomerMembership`, `MembershipStatus`, request types
- Endpoints: create plan, list plans, subscribe, renew, cancel, status
- Storage: all queries exist
- Helpers: subscribe, renew, cancel, get_status with auto-expiry

## What Needs Building

### Backend (Rust)
1. Wire multiplier into earn flow: in `do_process_earn()`, after rule evaluation, query membership + loyalty tier, take higher multiplier, apply to earning amounts
2. Potentially adjust bucket_type to `MembershipBenefit` for the multiplier bonus portion

### Frontend (SvelteKit)
1. New `memberships` module at `src/lib/client/modules/memberships/`
2. Admin page at `src/routes/admin/memberships/+page.svelte`
3. Sidebar nav item
4. Storefront: membership status on profile bar + rewards page

## Success Criteria

- [ ] Merchant can create/edit/deactivate membership plans
- [ ] Merchant can assign a customer to a plan (1-year expiry)
- [ ] Merchant can view all memberships and cancel them
- [ ] Earn flow applies the higher of membership/loyalty multiplier
- [ ] Storefront shows membership badge with plan name and days remaining
- [ ] "Memberships" appears in admin sidebar

## Dependencies

- Existing earn service (`src/services/earn/`)
- Existing loyalty service for tier multiplier lookup
- Existing rules service for `EvaluationResult`
