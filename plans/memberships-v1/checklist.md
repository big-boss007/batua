# Implementation Checklist

## Phase 2: Setup
- [ ] Create `frontend/src/lib/client/modules/memberships/` module directory
- [ ] Create barrel `index.ts`
- [ ] Add "Memberships" to admin sidebar nav
- [ ] Create admin route `routes/admin/memberships/+page.svelte` (stub)

## Phase 3: Type Definitions
- [ ] Create `memberships/types.ts` with all types
- [ ] Export types from barrel

## Phase 5: API Integration
- [ ] Create `memberships/remote.ts` with decoder functions
- [ ] Implement: createPlan, listPlans, subscribeMembership, cancelMembership, getMembershipStatus
- [ ] **Backend**: Add `GET /earn/memberships/subscribers/{merchant_id}` endpoint
- [ ] Implement: listSubscribers in remote.ts
- [ ] Export from barrel

## Phase 7: UI Components
- [ ] Create PlanForm.svelte (create/edit plan)
- [ ] Create AssignForm.svelte (assign customer to plan)
- [ ] Build admin memberships page — plans table + subscribers table
- [ ] Update storefront ProfileBar — membership badge
- [ ] Update storefront +page.svelte — fetch membership status

## Phase 8: Integration (Backend)
- [ ] Wire membership multiplier query into `do_process_earn()`
- [ ] Wire loyalty tier multiplier query into `do_process_earn()`
- [ ] Apply `max(membership_mult, tier_mult)` to evaluation results
- [ ] Add `list_memberships_by_merchant` storage query
- [ ] Add `list_subscribers` handler + route

## Verification
- [ ] `cargo check` passes
- [ ] `npx svelte-check --threshold error` passes
- [ ] Admin: can create a membership plan
- [ ] Admin: can assign customer to plan
- [ ] Admin: can view subscribers and cancel
- [ ] Storefront: membership badge shows for active member
- [ ] Earn flow: cashback multiplied for active member
- [ ] Earn flow: higher of membership/tier multiplier used
- [ ] Membership expires after 1 year (verify status endpoint)
