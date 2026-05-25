# Campaigns Final Design — Overview

## Goal

Implement the complete campaigns UI per `docs/campaigns-final-design.html`. The design has 7 states. Some are partially built; several are missing entirely.

## Current State

| Component | Exists? | Gap |
|---|---|---|
| `CampaignsList.svelte` | Yes | Missing: linked rule, progress bar, impact stats, click-to-detail, status border colors, "Starts in X days" |
| `FestiveTemplateGrid.svelte` | Yes | Mostly matches design (3-col grid, meta). Minor: template grid should be 3-col not 4-col auto-fill |
| `CampaignForm.svelte` | Yes | Missing: earning preview section. Has overlap detection. |
| `CampaignDetailModal` | **No** | Entire component needed |
| `StackingConfigModal` | **No** | Entire component needed |
| `EarnFormulaBanner` | **No** | Entire component needed |
| Campaigns page (`+page.svelte`) | Yes | Missing: earn banner, Settings button, + Create Campaign button, custom create flow, detail modal trigger |

## Scope

### In scope
- All 7 design states: Main Active, Main Empty, Create Custom, Create Template, Detail Modal, Stacking Config, Overlap Warning
- New components: `EarnFormulaBanner`, `CampaignDetailModal`, `StackingConfigModal`
- Enhanced `CampaignsList` with rule, progress, impact, click handler
- Enhanced `CampaignForm` with earning preview
- Enhanced campaigns page with all buttons/modals
- Types: `Campaign` needs `base_rule_id` field; may need campaign performance type

### Out of scope
- Backend API changes (all endpoints already exist)
- Loyalty page integration (already has campaigns tab)

## Success Criteria
- All 7 design states render correctly
- Campaign cards show linked rule, progress bar, impact stats
- Click campaign card opens detail modal
- "+ Create Campaign" opens custom create form
- Settings opens stacking config modal
- Earn formula banner shows when active campaign exists
- Overlap warning shows correctly in create form
- `svelte-check` passes with 0 errors

## Dependencies
- Existing API endpoints: `/campaigns/calendar`, `/campaigns/create`, `/campaigns/from-template`, `/campaigns/{id}/deactivate`, `/admin/merchants/{id}/campaign-config`
- `@juspay/svelte-ui-components` for Button, Pill, Progress, Modal if available
