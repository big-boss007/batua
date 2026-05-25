# Storage Layer

## Objective
Add migration for the new column and update storage queries.

## Migration
**File:** `migrations/XXXXXX_referral_code_creation_trigger.sql`

```sql
ALTER TABLE referral_programs
ADD COLUMN code_creation_trigger TEXT NOT NULL DEFAULT 'on_registration';
```

## Storage Updates

### `src/services/referrals/storage.rs`

**Update all `SELECT` queries for `referral_programs`** to include `code_creation_trigger`:
- `get_referral_program()` — add field to SELECT
- `create_referral_program()` — add field to INSERT
- `update_referral_program()` — add field to UPDATE SET clause

### New Function: `get_program_trigger_setting()`
A lightweight query used by the earn service to check the trigger without fetching the full program:

```rust
pub async fn get_active_program_trigger(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Option<CodeCreationTrigger>, AppError> {
    // SELECT code_creation_trigger FROM referral_programs
    // WHERE merchant_id = $1 AND is_active = true
}
```

### Existing Function: `create_referral_code()`
Already exists at storage.rs:102-132. No changes needed — the earn service will call it directly.

### Existing Function: `get_customer_referral_code()`
Already exists at storage.rs:153-174. Used for idempotency check (does code already exist?).
