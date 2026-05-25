# Auto-Create Referral Codes

## Goal
Automatically create referral codes for customers based on a merchant-configurable trigger: **on registration** (default) or **on first purchase**. Add this setting to the referral program setup wizard and make it editable later.

## Scope

### In Scope
- Add `code_creation_trigger` field to `referral_programs` table (`on_registration` | `on_first_purchase`)
- Auto-create referral codes in the earn flow (`process_earn`) based on the trigger setting
- Update the referral program setup wizard UI to include this setting
- Show tradeoff advice on the confirmation screen
- Allow merchants to change this setting after initial setup
- Idempotency: skip if customer already has a code for that merchant

### Out of Scope
- Bulk retroactive code creation for existing customers
- Referral code auto-creation via admin bulk import
- Changes to the referral conversion flow

## Key Design Decisions

### Trigger Points in the Earn Flow
Both triggers fire during `do_process_earn()` in `src/services/earn/helpers.rs`:
- **On registration:** After `resolve_or_create()` returns `is_new = true` (new customer created)
- **On first purchase:** After earn processing completes and `order_stats.is_none()` (first order)

In practice, for Shopify-sourced customers these are nearly the same moment. The difference matters for:
- Admin-created customers (registration trigger creates code immediately)
- Future customer import features

### Guard Conditions
Before auto-creating a code:
1. Merchant has an active referral program (`is_active = true`)
2. Customer doesn't already have a code for this merchant (idempotency)
3. The trigger matches the program's `code_creation_trigger` setting

### Code Generation
Use existing `generate_referral_code(customer_name)` for vanity-style codes (e.g., "KABI98").

## Files to Modify

### Backend
- `migrations/XXXXXX_referral_code_creation_trigger.sql` — new migration
- `src/services/referrals/types.rs` — add `CodeCreationTrigger` enum
- `src/services/referrals/storage.rs` — update program queries to include new field
- `src/services/earn/helpers.rs` — add auto-creation logic in `do_process_earn()`

### Frontend
- `frontend/src/routes/admin/referrals/+page.svelte` — add trigger setting to wizard + edit form
- `frontend/src/lib/client/modules/referrals/types.ts` — add field to types
- `frontend/src/lib/client/modules/referrals/remote.ts` — update program API calls

## Success Criteria
- [ ] New customers get referral codes automatically based on merchant's chosen trigger
- [ ] Wizard shows the setting with clear tradeoff explanation
- [ ] Merchant can change the setting after setup
- [ ] No duplicate codes are created (idempotency)
- [ ] Existing referral flows (conversion, rewards) are unaffected
