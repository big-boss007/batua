# Implementation Checklist

## Phase 3: Type Definitions
- [x] Add `base_rule_id` to `Campaign` type
- [x] Add `CampaignPerformance` type

## Phase 7: UI Components
- [x] Enhance `CampaignsList.svelte` — 2-col grid, status borders, rule, progress, impact, click handler
- [x] Create `EarnFormulaBanner.svelte`
- [x] Create `CampaignDetailModal.svelte`
- [x] Create `StackingConfigModal.svelte`
- [x] Enhance `CampaignForm.svelte` — add earning preview
- [x] Update `FestiveTemplateGrid.svelte` — 3-col grid
- [x] Update `ui/index.ts` — add new exports

## Phase 8: Page Integration
- [x] Enhance `+page.svelte` — earn banner, buttons, all modals, stacking config fetch

## Verification
- [x] `npx svelte-check --threshold error` passes with 0 errors
- [ ] All 7 design states render correctly in browser
