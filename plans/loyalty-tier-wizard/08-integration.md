# Phase 8: Integration

## Objective
Wire everything together and test end-to-end.

## Tasks
- Update `customers/ui/index.ts` barrel to export `TierWizard`
- Update `customers/index.ts` barrel to export `updateTier`, `deleteTier`
- Update `+page.svelte` to use wizard vs existing view based on program state
- Test full flow in browser at localhost:5174/admin/loyalty

## Validation
- `npx svelte-check --threshold error` passes
- `cargo check` passes
- Wizard creates program + tiers successfully
- Existing tiers can be edited and deleted
- Page refresh preserves all data
