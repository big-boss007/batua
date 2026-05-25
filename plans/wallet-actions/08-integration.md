# Phase 8: Integration

## Objective

Wire everything together, export through barrels, and verify end-to-end.

## Tasks

### 1. Barrel Exports (customers/index.ts)

Export new types:
- `WalletActionType`, `WalletUnitType`, `WalletActionResult`
- `AddRequest`, `RemoveRequest`, `ExpireRequest`

Export new remote functions:
- `addCredit`, `removeBalance`, `expireBalance`

### 2. Backend Route Registration

Ensure new endpoints are mounted:
- `POST /admin/debit` → `admin_debit` handler
- `POST /admin/force-expire` → `admin_force_expire` handler

Both must go through existing admin auth middleware.

### 3. Customer Detail Refresh Flow

After a successful wallet action:
1. Modal calls `onSuccess` callback
2. Parent (customers page) re-fetches customer detail
3. Customer detail re-renders with updated balances
4. Modal closes

### 4. End-to-End Testing (Manual)

Test matrix:

| Operation | Unit | Test Case |
|-----------|------|-----------|
| Add | Cash | Add ₹500 Store Credit, verify balance increases |
| Add | Points | Add 200 Courtesy Points, verify balance + INR equiv |
| Add | Points | Add enough to trigger tier upgrade preview |
| Remove | Cash | Remove ₹100 from Store Credit, verify decrease |
| Remove | Cash | Try to remove more than balance, verify error |
| Remove | Points | Remove 500 Reward Points, verify tier impact warning |
| Expire | Cash | Expire Store Credit bucket, verify zeroed |
| Expire | Cash | Expire multiple buckets simultaneously |
| Expire | Points | Expire Reward Points, verify tier downgrade warning |
| Error | Any | Disconnect backend, verify error state + retry |
| Validation | Any | Submit empty form, verify all validation errors |
| High-value | Add | Enter ₹10,000+, verify warning banner |

### 5. Code Quality

- Run `npx svelte-check --threshold error`
- Run `cargo check`
- Run `cargo test` (including tracing lint)
- Run svelte-autofixer on all modified .svelte files

## Outputs

- Feature fully functional end-to-end
- All barrel exports in place
- Backend routes registered and authenticated
- Manual test matrix passed

## Validation

- All 12 test cases in the matrix pass
- No TypeScript or Rust compilation errors
- Browser verification against design mockup
