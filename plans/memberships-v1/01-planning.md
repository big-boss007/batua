# Phase 1: Planning

## Status: COMPLETE

Design decisions finalized:

1. **Multiplier logic**: `effective_multiplier = max(loyalty_tier_multiplier, membership_multiplier)` — no stacking
2. **Expiry**: 1 year from assignment, auto-expired by existing `get_membership_status()` logic
3. **Admin UI**: Single page with tabs or sections for Plans and Subscribers
4. **Storefront**: Badge in ProfileBar component (already built in storefront redesign)
5. **No new backend endpoints needed** — all CRUD exists, just need earn flow wiring
6. **Bucket type**: Multiplier bonus credits go into same bucket as the base earn rule (not a separate MembershipBenefit bucket), since the multiplier is applied to the whole reward
