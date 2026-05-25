# Phase 3: Type Definitions

## Objective
Add types for update/delete tier operations.

## Backend (Rust)

### `src/services/loyalty/types.rs`
Add `UpdateTierRequest`:
```rust
#[derive(Debug, Deserialize)]
pub struct UpdateTierRequest {
    pub name: Option<String>,
    pub rank: Option<i32>,
    pub threshold: Option<f64>,
    pub earn_rate_multiplier: Option<f64>,
    pub benefits: Option<serde_json::Value>,
}
```

## Frontend (TypeScript)

No new types needed — existing `LoyaltyTier` type already covers all fields. The update call will use a `Partial<LoyaltyTier>` style body.

## Validation
- `cargo check` passes
- `npx svelte-check --threshold error` passes
