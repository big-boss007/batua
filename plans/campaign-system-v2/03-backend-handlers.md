# Phase 3: Backend Handlers & Routes

## Objective
Add API endpoints for direct campaign creation, stacking config, and campaign performance.

## New Endpoints

### `POST /campaigns/create` → `handler::create_campaign_direct`
- Takes `CreateCampaignDirectRequest` body
- Validates: base_rule_id exists and belongs to merchant, multiplier > 0, ends_at > starts_at
- Creates campaign in DB
- Returns 201 with created Campaign

### `GET /campaigns/{campaign_id}/performance` → `handler::get_campaign_performance`
- Returns `CampaignPerformance` (orders, customers, extra points, extra value)
- Already partially exists — verify and complete

### `GET /admin/merchants/{merchant_id}/campaign-config` → `handler::get_campaign_config`
- Returns `CampaignStackingConfig`

### `PUT /admin/merchants/{merchant_id}/campaign-config` → `handler::update_campaign_config`
- Takes `CampaignStackingConfig` body
- Validates stacking_mode is one of: multiplicative, best_of, additive
- Validates max_multiplier > 0

### `POST /campaigns/{campaign_id}/deactivate` → `handler::deactivate_campaign`
- Sets `is_active = false` on the campaign
- Returns updated Campaign

## Route Registration
- Add new routes to `src/services/campaigns/mod.rs` or `src/services/rules/mod.rs` depending on existing structure
- Add config routes to `src/services/admin/mod.rs`

## Validation
- `cargo check` passes
- curl tests for all new endpoints
- Direct campaign creation works without template
- Stacking config persists across requests
