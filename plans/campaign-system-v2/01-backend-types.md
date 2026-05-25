# Phase 1: Backend Types & Migration

## Objective
Add types for direct campaign creation, stacking config, and campaign performance response.

## Database Migration

### `merchants` table additions:
```sql
ALTER TABLE merchants
  ADD COLUMN campaign_stacking_mode TEXT NOT NULL DEFAULT 'multiplicative',
  ADD COLUMN max_campaign_multiplier FLOAT NOT NULL DEFAULT 10.0;
```

## Types

### `src/services/rules/types.rs` — new/modified:

**Already exists**: `CreateCampaignRequest` (used internally). Needs to be exposed as an API type:
```rust
#[derive(Debug, Deserialize)]
pub struct CreateCampaignDirectRequest {
    pub merchant_id: Uuid,
    pub name: String,
    pub campaign_type: String,
    pub base_rule_id: Uuid,      // required (not optional like internal type)
    pub multiplier: f64,          // required (not optional)
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}
```

**New**: Campaign performance response:
```rust
#[derive(Debug, Serialize)]
pub struct CampaignPerformance {
    pub campaign_id: Uuid,
    pub total_orders: i64,
    pub unique_customers: i64,
    pub total_extra_points: f64,
    pub total_extra_value: f64,
}
```

**New**: Stacking config (read from merchant):
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignStackingConfig {
    pub stacking_mode: String,       // multiplicative | best_of | additive
    pub max_multiplier: f64,         // default 10.0
    pub one_campaign_per_rule: bool, // enforced by best-campaign-wins logic
}
```

### Frontend types (`frontend/src/lib/client/modules/rules/types.ts`):

Add to existing Campaign type:
```typescript
export type CreateCampaignRequest = {
  merchant_id: string;
  name: string;
  campaign_type: string;
  base_rule_id: string;
  multiplier: number;
  starts_at: string;
  ends_at: string;
};

export type CampaignPerformance = {
  campaign_id: string;
  total_orders: number;
  unique_customers: number;
  total_extra_points: number;
  total_extra_value: number;
};

export type CampaignStackingConfig = {
  stacking_mode: string;
  max_multiplier: number;
};
```

## Validation
- `cargo check` passes
- `npx svelte-check --threshold error` passes
