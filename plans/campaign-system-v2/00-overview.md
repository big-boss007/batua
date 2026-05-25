# Campaign System V2 — Overview

## Goal

Enhance the campaign system with:
1. **Custom campaign creation** — merchants can create campaigns from scratch (not just from templates)
2. **Configurable stacking mode** — merchant chooses how campaign × tier multipliers interact (multiplicative / best-of / additive)
3. **Max multiplier cap** — safety limit on total effective multiplier
4. **Best-campaign-wins overlap** — when multiple campaigns apply to the same rule, highest multiplier wins
5. **Campaign detail modal** — view full info, linked rule, earn formula, performance stats, deactivate
6. **Improved campaign cards** — linked rule name, duration progress, impact stats
7. **Earn formula banner** — visual breakdown shown when a campaign is active

## Scope

### In scope
- **Backend**: New `CreateCampaignRequest` endpoint (direct, not template-only), stacking mode config on merchant, fix `.find()` → `.max_by()` for campaign overlap, max multiplier cap enforcement
- **Backend**: Campaign performance endpoint (if not already complete)
- **Frontend**: Custom campaign creation form modal
- **Frontend**: Stacking mode configuration UI (in Settings or Campaigns tab)
- **Frontend**: Campaign detail modal on card click
- **Frontend**: Earn formula banner when active campaign exists
- **Frontend**: Improved campaign cards with linked rule, progress, impact
- **Frontend**: Overlap warning during creation

### Out of scope
- Campaign scheduling/automation (cron-based)
- Campaign analytics dashboard (separate feature)
- A/B testing of campaigns
- Campaign templates CRUD (templates remain hardcoded)

## Success Criteria

- Merchant can create a campaign without picking a template first
- Merchant can configure stacking mode (multiplicative/best-of/additive) and max cap
- When 2 campaigns overlap on same rule, the higher multiplier wins automatically
- Campaign cards show linked rule name, duration progress bar, and impact stats
- Clicking a campaign card opens a detail modal with full info + deactivate option
- Active campaign shows earn formula banner at the top of campaigns tab
- Overlap warning shown when creating a campaign that conflicts with existing one

## Dependencies

- Existing campaign service: `src/services/campaigns/`
- Existing rules service: `src/services/rules/`
- Existing earn service multiplier logic: `src/services/earn/helpers.rs`
- Existing frontend campaign components: `frontend/src/lib/client/modules/rules/`
- Merchant settings/config (for stacking mode storage)

## Architecture Notes

### Stacking mode storage
Add `campaign_stacking_mode` and `max_campaign_multiplier` to the `merchants` table (or a merchant_config table). Values: `multiplicative` (default), `best_of`, `additive`.

### Campaign overlap resolution
In `rules/helpers.rs`, change:
```rust
// Before: .find() — returns first match (arbitrary)
let matching_campaign = active_campaigns.iter().find(|c| c.base_rule_id == Some(rule.id));

// After: .max_by() — returns highest multiplier
let matching_campaign = active_campaigns.iter()
    .filter(|c| c.base_rule_id == Some(rule.id))
    .max_by(|a, b| a.multiplier.unwrap_or(1.0).partial_cmp(&b.multiplier.unwrap_or(1.0)).unwrap());
```

### Stacking mode application
In `earn/helpers.rs`, the effective multiplier calculation changes based on stacking mode:
```rust
let effective_mult = match stacking_mode {
    "multiplicative" => tier_mult * campaign_mult,  // current behavior
    "best_of" => tier_mult.max(campaign_mult),
    "additive" => 1.0 + (tier_mult - 1.0) + (campaign_mult - 1.0),
    _ => tier_mult * campaign_mult,
};
let effective_mult = effective_mult.min(max_cap);
```

Currently the campaign multiplier is applied in `rules/helpers.rs` and the tier multiplier in `earn/helpers.rs` — these need to be unified into a single place for stacking mode to work correctly.
