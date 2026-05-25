# Implementation Checklist

## Phase 1: Types
- [ ] Add `UpdateCustomerRequest` to `src/services/admin/types.rs`

## Phase 2: Storage
- [ ] Add `update_customer` function to `src/services/admin/storage.rs`

## Phase 3: Handler
- [ ] Add `update_customer` handler to `src/services/admin/handler.rs`

## Phase 4: Router
- [ ] Add PUT route to `src/services/admin/mod.rs`

## Phase 5: Frontend
- [ ] Add `updateCustomer` to `frontend/src/lib/client/modules/customers/remote.ts`
- [ ] Export from `frontend/src/lib/client/modules/customers/index.ts`
- [ ] Add edit form UI to `CustomerDetail.svelte` (Option B design)

## Phase 6: Testing
- [ ] Verify in browser — all states (resting, editing, after save)
- [ ] Create HTML report with screenshots
