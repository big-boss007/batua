# Phase 8: Page Integration

## Objective
Wire all new components into the campaigns page.

## Tasks

### 1. Enhance `+page.svelte`

Current page is minimal. Needs:

- **Earn formula banner** at top (when active campaign exists)
- **Section header** with "Settings" + "+ Create Campaign" buttons
- **CampaignsList** with `rules` and `onSelect` props
- **Custom create modal** (separate from template create) — triggered by "+ Create Campaign" button
- **Detail modal** — triggered by clicking a campaign card
- **Stacking config modal** — triggered by Settings button
- **Fetch stacking config** on load
- **Handle deactivate** callback

### 2. State additions to page
```
selectedCampaign: Campaign | null  // for detail modal
showCreateModal: boolean  // for custom create
showSettingsModal: boolean  // for stacking config
stackingConfig: CampaignStackingConfig  // loaded from API
```

### 3. Handler functions
- `handleSelectCampaign(campaign)` — open detail modal
- `handleCreateDirect(req)` — create custom campaign via API
- `handleDeactivate(campaignId)` — deactivate + remove from store
- `handleOpenSettings()` / `handleSaveSettings(config)` — stacking config
- `handleOpenCreate()` / `handleCancelCreate()` — custom create modal

## Files
- `frontend/src/routes/admin/campaigns/+page.svelte`
