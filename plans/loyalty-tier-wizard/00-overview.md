# Loyalty Tier Wizard — Overview

## Goal

Redesign the loyalty tier management on `/admin/loyalty` to support:
1. **Editing existing tiers** — currently tiers can only be created, never updated or deleted
2. **Wizard-style tier setup** — guided, step-by-step experience for setting up tiers instead of the current "click Add Tier → fill form → repeat" pattern

## Scope

### In scope
- Backend: add `update_tier` and `delete_tier` endpoints
- Frontend: inline-editable tier rows (click tier row → expand to edit form)
- Frontend: wizard view for first-time tier setup (when no tiers exist)
- Frontend: delete tier with confirmation

### Out of scope
- Changing program evaluation criteria or evaluation period (separate task)
- Tier benefits structured schema (keep JSON blob for now)
- Tier downgrade policies, grace periods, expiry
- Storefront tier display changes

## Success Criteria

- Merchant can edit any field of an existing tier (name, rank, threshold, multiplier, benefits)
- Merchant can delete a tier (with confirmation)
- First-time setup shows a wizard: Step 1 = program name + criteria, Step 2 = add tiers with presets, Step 3 = review & confirm
- Returning merchants see the existing view with inline edit capability
- All changes persist via API and survive page refresh

## Dependencies

- Existing loyalty service (backend): `src/services/loyalty/`
- Existing customers module (frontend): `src/lib/client/modules/customers/`
- `@juspay/svelte-ui-components` for UI elements
