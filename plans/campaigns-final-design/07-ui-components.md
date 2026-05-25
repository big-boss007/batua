# Phase 7: UI Components — THE MAIN PHASE

## Objective
Build/enhance all campaign UI components to match the 7 design states.

## Task 1: Enhance `CampaignsList.svelte`

Currently: simple cards with name, status badge, type, dates, multiplier.

Design requires:
- **2-column grid** (not auto-fill)
- **Status-based border colors**: active → green border + light green bg, upcoming → indigo border, ended → 0.6 opacity
- **Linked rule row**: "Rule: {rule_name}" with pill styling below the meta
- **Progress bar** for active campaigns: "Progress — 36% · 58 days left"
- **Impact stats** for active campaigns: Orders, Extra Points (+48.2K), Extra ₹ (₹12,050) in 3-col grid
- **"Starts in X days"** text for upcoming campaigns
- **Click handler**: `onSelect(campaign)` callback for opening detail modal
- Remove `campaign_type` row (design doesn't show it)

### Props change
```
campaigns: Array<Campaign>
rules: Array<Rule>  // NEW — to resolve rule names
onSelect: (campaign: Campaign) => void  // NEW — click handler
```

## Task 2: New `EarnFormulaBanner.svelte`

Shows when active campaign exists. Design:
- Green gradient background (#f0fdf4 → #eff6ff)
- Title: "Active Campaign Earning Formula"
- Formula blocks: Base (100) × Campaign (2x, blue) × Tier (1.25x, purple) = Effective (250, dark green bg)
- Example text below

### Props
```
multiplier: number
campaignName: string
stackingMode: string
maxCap: number
```

## Task 3: New `CampaignDetailModal.svelte`

Full detail view when clicking a campaign card. Design:
- Modal with header (title + close button), body, no footer (deactivate is in body)
- Campaign name + status badge
- Info grid: Multiplier (purple mono), Duration (date range)
- Linked Rule section: rule name + type pill + status pill + event description
- Progress bar: "36% elapsed · 58 days remaining"
- Earn formula (reuse EarnFormulaBanner in compact form)
- Performance grid: Orders, Customers, Extra Points, Extra ₹
- Deactivate button (danger, bottom)

### Props
```
campaign: Campaign
rules: Array<Rule>
stackingConfig: CampaignStackingConfig
onDeactivate: (campaignId: string) => void
onClose: () => void
```

## Task 4: New `StackingConfigModal.svelte`

Settings modal for campaign stacking. Design:
- Modal with header "Campaign Settings", body, footer (Cancel + Save)
- Section 1: "Multiplier Stacking Mode" — 3 radio options:
  - Multiplicative (recommended): "Campaign and tier multipliers multiply together" + example "Gold 2x × Campaign 2x = 4x total"
  - Best-of: "Customer gets the higher of campaign or tier multiplier" + example
  - Additive: "Campaign bonus is added to tier multiplier" + example
- Section 2: "Safety Limits" — Max Effective Multiplier input (number, 1-50)
- Note at bottom: "When multiple campaigns overlap..."

### Props
```
config: CampaignStackingConfig
onSave: (config: CampaignStackingConfig) => void
onCancel: () => void
```

## Task 5: Enhance `CampaignForm.svelte`

Add earning preview section at the bottom of the form (before actions):
- "Earning Preview" label
- Compact formula: Base (100) × Campaign ({multiplier}x) × Tier (1x–2x) = Effective ({multiplier}x–{multiplier*2}x)
- "Stacking: {mode} · Max cap: {maxCap}x" text

## Task 6: Enhance `FestiveTemplateGrid.svelte`

Minor: change grid from 4-col auto-fill to 3-col (`1fr 1fr 1fr`), matching design.

## Files
- `frontend/src/lib/client/modules/rules/ui/CampaignsList.svelte` (enhance)
- `frontend/src/lib/client/modules/rules/ui/EarnFormulaBanner.svelte` (new)
- `frontend/src/lib/client/modules/rules/ui/CampaignDetailModal.svelte` (new)
- `frontend/src/lib/client/modules/rules/ui/StackingConfigModal.svelte` (new)
- `frontend/src/lib/client/modules/rules/ui/CampaignForm.svelte` (enhance)
- `frontend/src/lib/client/modules/rules/ui/FestiveTemplateGrid.svelte` (minor)
- `frontend/src/lib/client/modules/rules/ui/index.ts` (add exports)
