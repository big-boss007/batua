# Phase 5: Frontend UI Components

## Objective
Build the campaign creation form, stacking config UI, detail modal, earn formula banner, and improved campaign cards.

## Component Changes

### A. Create Campaign Form — `CampaignCreateForm.svelte` (NEW)
- **Replaces/augments**: Existing `CampaignForm.svelte` which only works with templates
- **Two modes**: "Custom" (blank form) and "From Template" (pre-filled)
- **Fields**:
  - Campaign name (text input)
  - Campaign type (select: multiplier / bonus)
  - Multiplier (number, 1-20, step 0.5)
  - Linked reward rule (select from active rules)
  - Start date / end date (date inputs)
- **Earning preview**: Shows formula blocks at bottom
- **Overlap detection**: Checks existing campaigns for same rule + date overlap, shows warning
- **Props**: `rules: Rule[]`, `existingCampaigns: Campaign[]`, `template: FestiveTemplate | null`, `onSave`, `onCancel`

### B. Campaign Stacking Config — `CampaignSettings.svelte` (NEW)
- **Three radio options**: Multiplicative (recommended), Best-of, Additive
- Each with description + formula example
- **Safety limits**:
  - Max effective multiplier (number input)
  - One campaign per rule toggle (informational — enforced by best-campaign-wins)
  - Overlap warning toggle
- **Props**: `config: CampaignStackingConfig`, `onSave`, `onCancel`
- **Placement**: Modal opened from a "⚙ Settings" button in campaigns tab header

### C. Campaign Detail Modal — inline in `+page.svelte`
- **Triggered by**: clicking a campaign card
- **Sections**:
  - Header: name + status badge
  - Detail grid: type, multiplier, start/end dates
  - Linked rule: rule name, type badge, event type
  - Duration progress bar
  - Earn formula (compact)
  - Performance stats: orders, customers, extra points, extra ₹
  - Deactivate button (with confirmation)
- **State**: `selectedCampaign: Campaign | null`

### D. Earn Formula Banner — inline in `+page.svelte`
- **Condition**: shown when any campaign is active
- **Content**: visual formula blocks: Base × Campaign × Tier = Effective
- **Stacking mode label** shown at bottom
- Uses the first active campaign's multiplier for the example

### E. Improved Campaign Cards — modify `CampaignsList.svelte`
- Add: linked rule name (need to pass rules to component)
- Add: duration progress bar (elapsed %)
- Add: impact stats row (orders, extra points, extra ₹) — fetched via performance API
- Add: "Starts in X days" for upcoming
- Add: click handler to open detail modal

### F. Overlap Warning — inside CampaignCreateForm
- When user selects dates + rule, check existing campaigns for overlap
- If overlap found: amber warning box with conflicting campaign info
- "Create Anyway" (secondary) instead of "Create Campaign" (primary)

## Page Changes (`+page.svelte` Campaigns tab)

- Add `+ Create Campaign` button in section header
- Add state: `showCreateForm`, `selectedCampaign`, `showSettings`
- Add `createCampaignDirect` handler
- Add `deactivateCampaign` handler
- Load campaign performance for active campaigns on mount
- Load stacking config on mount

## Validation
- `npx svelte-check --threshold error` passes
- Custom campaign creation works end-to-end
- Template creation still works
- Detail modal opens on card click
- Stacking config saves and persists
- Earn banner shows with correct formula
- Overlap warning appears correctly
