# Phase 5: API Integration

## Objective
Add backend endpoints and frontend API calls for update and delete tier.

## Backend

### New endpoints in `src/services/loyalty/mod.rs`:
- `PUT /loyalty/tiers/{tier_id}` → `handler::update_tier`
- `DELETE /loyalty/tiers/{tier_id}` → `handler::delete_tier`

### Storage (`src/services/loyalty/storage.rs`):
- `update_tier(pool, tier_id, req) -> Result<LoyaltyTier>`
- `delete_tier(pool, tier_id) -> Result<()>`

### Handler (`src/services/loyalty/handler.rs`):
- `update_tier` — takes Path(tier_id) + Json(UpdateTierRequest), returns updated tier
- `delete_tier` — takes Path(tier_id), returns 204 No Content

## Frontend

### `remote.ts` — add two functions:
```typescript
async function updateTier(
  tierId: string,
  body: Partial<Omit<LoyaltyTier, 'id'>>
): Promise<APIResult<LoyaltyTier>>

async function deleteTier(tierId: string): Promise<APIResult<null>>
```

### Export from barrel (`index.ts`)

## Validation
- `cargo check` passes
- Backend endpoints respond correctly via curl
- Frontend calls work from browser
