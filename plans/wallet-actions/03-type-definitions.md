# Phase 3: Type Definitions

## Objective

Define all TypeScript and Rust types needed for the 3 wallet operations.

## Tasks

### Frontend Types (customers/types.ts)

```typescript
type WalletActionType = 'add' | 'remove' | 'expire';
type WalletUnitType = 'cash' | 'points';

type WalletActionState = 'form' | 'confirm' | 'loading' | 'success' | 'error';

type AddRequest = {
  customer_id: string;
  bucket_type: string;
  amount: number;
  expiry_days: number | null;
  reason_category: string;
  reason_text: string;
  reference: string | null;
};

type RemoveRequest = {
  customer_id: string;
  bucket_type: string;
  amount: number;
  reason_category: string;
  reason_text: string;
  reference: string | null;
};

type ExpireRequest = {
  customer_id: string;
  bucket_types: Array<string>;
  reason_category: string;
  reason_text: string;
  reference: string | null;
};

type WalletActionResult = {
  success: boolean;
  ledger_entry_id: string | null;
  entries_affected: number;
  amount_affected: number;
  new_balance: number;
  error: string | null;
};
```

### Backend Types (admin/types.rs)

```rust
pub struct AdminDebitRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    pub bucket_type: String,
    pub amount: f64,
    pub reason: String,
    pub reference: Option<String>,
    pub actor_id: String,
}

pub struct AdminExpireRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    pub bucket_types: Vec<String>,
    pub reason: String,
    pub reference: Option<String>,
    pub actor_id: String,
}

pub struct AdminDebitResult {
    pub success: bool,
    pub ledger_entry_id: Option<Uuid>,
    pub amount_debited: f64,
    pub new_balance: f64,
}

pub struct AdminExpireResult {
    pub success: bool,
    pub entries_expired: i32,
    pub amount_expired: f64,
    pub new_balance: f64,
}
```

## Outputs

- Frontend types added to `customers/types.ts`
- Backend types added to `admin/types.rs`
- Types exported through barrel files

## Validation

- `npx svelte-check --threshold error` passes
- `cargo check` passes
