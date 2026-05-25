# Phase 1: Planning

## Objective

Map out all files to create/modify, data flow, and API contracts before writing code.

## Files to Create

| File | Purpose |
|------|---------|
| `frontend/src/lib/client/modules/customers/ui/WalletActionModal.svelte` | Main modal component |
| `src/services/admin/handler.rs` (modify) | Add `admin_debit` and `admin_force_expire` handlers |
| `src/services/admin/types.rs` (modify) | Add `AdminDebitRequest`, `AdminExpireRequest` types |
| `src/services/admin/storage.rs` (modify) | Add `process_debit`, `process_force_expire` functions |
| `src/services/admin/helpers.rs` (modify) | Add helper logic for debit/expire |

## Files to Modify

| File | Change |
|------|--------|
| `frontend/src/lib/client/modules/customers/ui/CustomerDetail.svelte` | Add entry points (+ Add button, overflow menu) |
| `frontend/src/lib/client/modules/customers/remote.ts` | Add `addCredit`, `removeBalance`, `expireBalance` API functions |
| `frontend/src/lib/client/modules/customers/types.ts` | Add request/response types |
| `frontend/src/lib/client/modules/customers/index.ts` | Export new types and functions |
| `src/main.rs` (if needed) | Register new routes |

## API Contracts

### POST /admin/credit (existing — Add)
Uses existing `POST /admin/bulk-credit` with single customer_id.

### POST /admin/debit (new — Remove)
```
Request:  { merchant_id, customer_id, bucket_type, amount, reason, reference?, actor_id }
Response: { success, ledger_entry_id, new_balance }
```

### POST /admin/force-expire (new — Expire)
```
Request:  { merchant_id, customer_id, bucket_types: string[], reason, reference?, actor_id }
Response: { success, entries_expired, amount_expired, new_balance }
```

## Data Flow

```
CustomerDetail → Click "+ Add" or "..." menu
  → WalletActionModal opens (unit pre-selected)
    → User fills form → Preview shows impact
    → User confirms → API call
    → Success → Close modal, refresh customer detail
    → Error → Show error, preserve form
```

## Validation Rules

| Field | Add | Remove | Expire |
|-------|-----|--------|--------|
| Bucket | Required (radio) | Required (radio, balance > 0) | Required (checkbox, balance > 0) |
| Amount | Required, > 0 | Required, > 0, <= bucket balance | N/A (expires all) |
| Expiry days | Optional (default from policy) | N/A | N/A |
| Reason category | Required | Required | Required |
| Reason text | Required | Required | Required |
| Reference | Optional | Optional | Optional |

## Outputs

- Complete file list with expected changes
- API contracts agreed upon
- Validation rules defined

## Validation

- All stakeholders (design, backend, frontend) agree on scope
- API contracts match backend capabilities
