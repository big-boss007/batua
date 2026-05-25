# Phase 3: Type Definitions

## Objective
Add missing fields to Campaign type and add new types for campaign performance.

## Tasks

### 1. Extend `Campaign` type
Add `base_rule_id` field so cards can show linked rule.

### 2. Add `CampaignPerformance` type
For the detail modal impact stats:
```
total_orders: number
unique_customers: number
extra_points_issued: number
extra_value: number
```

## Files
- `frontend/src/lib/client/modules/rules/types.ts`
