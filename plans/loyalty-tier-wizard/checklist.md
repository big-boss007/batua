# Implementation Checklist

## Phase 3: Type Definitions
- [x] Add `UpdateTierRequest` to `src/services/loyalty/types.rs`
- [x] Add `UpdateProgramRequest` to `src/services/loyalty/types.rs`
- [x] Add `evaluation_period_days` to frontend `LoyaltyProgram` type

## Phase 5: API Integration
- [x] Add `update_tier` storage function
- [x] Add `delete_tier` storage function
- [x] Add `update_tier` handler
- [x] Add `delete_tier` handler
- [x] Add `update_program` handler
- [x] Register routes in `mod.rs` (PUT/DELETE tiers, PUT program)
- [x] Add `updateTier` to `remote.ts`
- [x] Add `deleteTier` to `remote.ts`
- [x] Add `updateProgram` to `remote.ts`
- [x] Export from barrel files

## Phase 7: UI Components — Part A (Editable Tiers)
- [x] Add edit/delete buttons to tier rows in `+page.svelte`
- [x] Wire inline TierForm for editing (expand on click)
- [x] Add `handleUpdateTier` function
- [x] Add `handleDeleteTier` function with confirmation
- [x] Add `onCancel` prop to TierForm

## Phase 7: UI Components — Part B (Wizard)
- [x] Create `TierWizard.svelte` with 3-step flow
- [x] Step 1: program name + criteria + evaluation period
- [x] Step 2: preset quick-add + custom form + tier list with edit/remove
- [x] Step 3: review with diff summary (reconfigure) or plain review (fresh)
- [x] Save logic: fresh (create program + tiers) and reconfigure (update/create/delete)
- [x] Conditionally render wizard vs existing view in page
- [x] "Reconfigure" button on existing view
- [x] Program summary card on existing view
- [x] Export TierWizard from barrel

## Phase 8: Integration
- [x] Update barrel exports
- [x] `cargo check` passes
- [x] `npx svelte-check --threshold error` passes (0 errors)

## Verification (pending browser testing)
- [ ] Test fresh wizard flow end-to-end in browser
- [ ] Test reconfigure flow in browser
- [ ] Test edit tier in browser (existing view)
- [ ] Test delete tier in browser (existing view)
