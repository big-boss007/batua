# Implementation Checklist

## Phase 1: Backend Types & Migration
- [ ] Write migration: add `campaign_stacking_mode` + `max_campaign_multiplier` to `merchants`
- [ ] Add `CreateCampaignDirectRequest` type
- [ ] Add `CampaignPerformance` response type
- [ ] Add `CampaignStackingConfig` type
- [ ] Add `campaign_multiplier: Option<f64>` to `EvaluationResult`
- [ ] Add frontend types: `CreateCampaignRequest`, `CampaignPerformance`, `CampaignStackingConfig`

## Phase 2: Backend Storage & Helpers
- [ ] Add `create_campaign_direct` storage function
- [ ] Add `get_campaign_performance` storage query
- [ ] Add `get_campaign_stacking_config` storage function
- [ ] Add `update_campaign_stacking_config` storage function
- [ ] Fix campaign overlap: `.find()` → `.max_by()` in `rules/helpers.rs`
- [ ] Remove `apply_campaign_multiplier` call — store multiplier in EvaluationResult instead
- [ ] Implement stacking mode logic in `earn/helpers.rs`
- [ ] Apply max multiplier cap

## Phase 3: Backend Handlers & Routes
- [ ] Add `POST /campaigns/create` handler
- [ ] Add/verify `GET /campaigns/{id}/performance` handler
- [ ] Add `GET /admin/merchants/{id}/campaign-config` handler
- [ ] Add `PUT /admin/merchants/{id}/campaign-config` handler
- [ ] Add `POST /campaigns/{id}/deactivate` handler
- [ ] Register all new routes
- [ ] `cargo check` passes

## Phase 4: Frontend API Integration
- [ ] Add `createCampaignDirect` API call
- [ ] Add `getCampaignPerformance` API call
- [ ] Add `deactivateCampaign` API call
- [ ] Add `getCampaignConfig` / `updateCampaignConfig` API calls
- [ ] Add decoders for new types
- [ ] Update barrel exports

## Phase 5: Frontend UI
- [ ] Create `CampaignCreateForm.svelte` (custom + template modes)
- [ ] Create `CampaignSettings.svelte` (stacking mode + safety limits)
- [ ] Add campaign detail modal to loyalty page
- [ ] Add earn formula banner (active campaign)
- [ ] Improve `CampaignsList.svelte` cards (rule, progress, impact)
- [ ] Add overlap warning in creation form
- [ ] Add `+ Create Campaign` button
- [ ] Add `⚙ Settings` button
- [ ] Wire all handlers in `+page.svelte`

## Verification
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] `npx svelte-check --threshold error` passes
- [ ] Custom campaign creation works (no template)
- [ ] Template campaign creation still works
- [ ] Campaign overlap: highest multiplier wins
- [ ] Stacking mode config saves and affects earn calculation
- [ ] Max cap enforced
- [ ] Detail modal opens on card click with all sections
- [ ] Earn banner shows correct formula
- [ ] Deactivate campaign works
- [ ] Overlap warning shows when dates conflict
