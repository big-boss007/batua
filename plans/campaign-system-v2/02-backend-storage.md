# Phase 2: Backend Storage & Helpers

## Objective
Add storage for direct campaign creation, campaign performance queries, stacking config reads, and fix campaign overlap logic.

## Storage Changes

### `src/services/rules/storage.rs`:

**New**: `create_campaign_direct(pool, req: CreateCampaignDirectRequest) -> Campaign`
- INSERT into campaigns with all fields, `is_active = true`, config as empty JSON
- Return created campaign

**New or existing**: `get_campaign_performance(pool, campaign_id: Uuid) -> CampaignPerformance`
- Join `campaign_snapshots` → `ledger_entries`
- Aggregate: COUNT DISTINCT orders (via payment_reference), COUNT DISTINCT customer wallets, SUM earning_units, SUM currency_equivalent
- Only entries where `campaign_snapshot_id` matches snapshots for this campaign

### `src/services/admin/storage.rs` (or merchant config):

**New**: `get_campaign_stacking_config(pool, merchant_id) -> CampaignStackingConfig`
- SELECT campaign_stacking_mode, max_campaign_multiplier FROM merchants WHERE id = $1

**New**: `update_campaign_stacking_config(pool, merchant_id, config) -> ()`
- UPDATE merchants SET campaign_stacking_mode = $2, max_campaign_multiplier = $3

## Helper Changes

### `src/services/rules/helpers.rs` — Fix campaign overlap:

```rust
// Change .find() to .max_by() for best-campaign-wins
let matching_campaign = active_campaigns
    .iter()
    .filter(|c| c.base_rule_id == Some(rule.id))
    .max_by(|a, b| {
        a.multiplier.unwrap_or(1.0)
            .partial_cmp(&b.multiplier.unwrap_or(1.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
```

### `src/services/earn/helpers.rs` — Stacking mode:

The campaign multiplier is currently applied in `rules/helpers.rs` (modifying `earning_unit` directly). The tier multiplier is applied separately in `earn/helpers.rs`. For stacking mode to work, we need to:

1. In `rules/helpers.rs`: store the campaign multiplier in the `EvaluationResult` but DON'T apply it to earning_unit yet
2. In `earn/helpers.rs`: apply both campaign and tier multipliers together based on stacking mode

**Modified flow**:
```rust
// earn/helpers.rs - do_process_earn()
let tier_mult = max(loyalty_mult, membership_mult);
let campaign_mult = eval.campaign_multiplier.unwrap_or(1.0);
let stacking_config = get_stacking_config(pool, merchant_id).await?;

let effective_mult = match stacking_config.stacking_mode.as_str() {
    "multiplicative" => tier_mult * campaign_mult,
    "best_of" => tier_mult.max(campaign_mult),
    "additive" => 1.0 + (tier_mult - 1.0) + (campaign_mult - 1.0),
    _ => tier_mult * campaign_mult,
};
let effective_mult = effective_mult.min(stacking_config.max_multiplier);

// Apply effective_mult to earning_unit
```

This requires adding `campaign_multiplier: Option<f64>` to `EvaluationResult` and removing the `apply_campaign_multiplier` call in `rules/helpers.rs`.

## Validation
- `cargo check` passes
- Existing tests pass
- Campaign overlap: when 2 campaigns on same rule are active, highest multiplier wins
