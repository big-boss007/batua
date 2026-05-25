# Type Definitions

## Objective
Add `CodeCreationTrigger` enum and update `ReferralProgram` struct.

## Backend Types

### New Enum: `CodeCreationTrigger`
**File:** `src/services/referrals/types.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum CodeCreationTrigger {
    OnRegistration,
    OnFirstPurchase,
}

impl Default for CodeCreationTrigger {
    fn default() -> Self {
        Self::OnRegistration
    }
}
```

### Update `ReferralProgram` Struct
Add field:
```rust
pub code_creation_trigger: CodeCreationTrigger,
```

### Update `CreateProgramRequest` / `UpdateProgramRequest`
Add optional field:
```rust
pub code_creation_trigger: Option<CodeCreationTrigger>,
```

## Frontend Types
**File:** `frontend/src/lib/client/modules/referrals/types.ts`

Add to `ReferralProgram` type:
```typescript
code_creation_trigger: 'on_registration' | 'on_first_purchase'
```

## Validation
- `code_creation_trigger` defaults to `on_registration` if not provided
- Only these two values are accepted; anything else returns 400
