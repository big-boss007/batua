# Wallet Actions — Add / Remove / Expire

## Goal

Build a unified WalletActionModal component accessible from the Customer Detail modal that lets admins perform 6 operations: Add Cash, Add Points, Remove Cash, Remove Points, Expire Cash, Expire Points. Backend endpoints for Remove and Expire need to be created (Add already exists via `POST /admin/bulk-credit`).

## Scope

### In Scope
- **Frontend:** WalletActionModal Svelte component with 3 tabs (Add / Remove / Expire)
- **Frontend:** Entry points on CustomerDetail.svelte (+ Add button, overflow menu for Remove/Expire)
- **Frontend:** API client functions for all 3 operations
- **Frontend:** Types for requests/responses
- **Backend:** `POST /admin/debit` endpoint (Remove operation)
- **Backend:** `POST /admin/force-expire` endpoint (Expire operation)
- **Backend:** Request/response types for new endpoints
- **Backend:** Storage layer for debit and expire operations

### Out of Scope
- Bulk operations (CSV import/export) — future phase
- Customer notification emails — future phase
- Audit log UI — existing ledger already tracks entries
- Per-entry undo/reversal — future phase

## Success Criteria

1. Admin can Add cash/points to any customer from Customer Detail modal
2. Admin can Remove cash/points from a specific bucket with balance > 0
3. Admin can Expire all remaining balance in selected bucket(s)
4. All operations create proper ledger entries
5. Remove/Expire show tier impact preview for points operations
6. Confirmation severity escalates: Add (single) < Remove (type amount) < Expire (type EXPIRE)
7. `cargo check` and `npx svelte-check --threshold error` pass
8. Operations verified end-to-end in browser

## Dependencies

- Existing `POST /admin/bulk-credit` endpoint (for Add)
- Existing CustomerDetail.svelte component
- Existing customers module (types, remote, utils)
- Existing ledger service (MovementType, BucketType, CreditState enums)

## Architecture Decisions

1. **Single modal component** with tab-based action switching (not 3 separate modals)
2. **Unit pre-selected from entry point** (Cash or Points row determines context)
3. **Backend uses existing ledger** — Remove creates `movement_type: Out` entries, Expire sets `state: Expired`
4. **No new database tables** — operations use existing `ledger_entries` table
5. **Sequential implementation** — shared files prevent parallelization

## Design Reference

- `docs/wallet-actions-design.html` — full mockup with 28 states
- `docs/competitive-analysis-wallet-actions.html` — competitive research informing design decisions
