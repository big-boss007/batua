# Implementation Checklist

## Phase 1: Planning
- [x] Map all files to create/modify
- [x] Define API contracts for Remove and Expire
- [x] Define validation rules per action
- [x] Create HTML design mockup (docs/wallet-actions-design.html)
- [x] Competitive analysis (docs/competitive-analysis-wallet-actions.html)

## Phase 2: Setup
- [x] SKIPPED — using existing module directories

## Phase 3: Type Definitions
- [ ] Add frontend types to `customers/types.ts` (WalletActionType, AddRequest, RemoveRequest, ExpireRequest, WalletActionResult)
- [ ] Add backend types to `admin/types.rs` (AdminDebitRequest, AdminExpireRequest, AdminDebitResult, AdminExpireResult)
- [ ] Export types through `customers/index.ts`
- [ ] `cargo check` passes
- [ ] `npx svelte-check` passes

## Phase 4: State Management
- [ ] Define component props interface
- [ ] Define all $state variables
- [ ] Define all $derived computations
- [ ] Define reason categories per action
- [ ] Define bucket filtering logic per action + unit

## Phase 5: API Integration
- [ ] Backend: `POST /admin/debit` handler + storage
- [ ] Backend: `POST /admin/force-expire` handler + storage
- [ ] Backend: Register routes in admin router
- [ ] Backend: `#[tracing::instrument]` on all new functions
- [ ] Frontend: `addCredit()` in customers/remote.ts
- [ ] Frontend: `removeBalance()` in customers/remote.ts
- [ ] Frontend: `expireBalance()` in customers/remote.ts
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] Manual curl test of new endpoints

## Phase 6: Utilities
- [ ] `getAddBuckets()` helper
- [ ] `getActionReasonPills()` helper
- [ ] `validateWalletAction()` helper
- [ ] Tier impact calculation (if applicable from data available)

## Phase 7: UI Components
- [ ] WalletActionModal.svelte — shell (tabs, customer banner)
- [ ] WalletActionModal.svelte — Add form (bucket, amount, expiry, reason, reference, notify)
- [ ] WalletActionModal.svelte — Remove form (bucket with balance, amount with max, reason)
- [ ] WalletActionModal.svelte — Expire form (multi-bucket checkboxes, reason)
- [ ] WalletActionModal.svelte — Preview boxes (green/red/amber)
- [ ] WalletActionModal.svelte — Tier impact preview (points only)
- [ ] WalletActionModal.svelte — Confirmation step (single / type-amount / type-EXPIRE)
- [ ] WalletActionModal.svelte — Loading state
- [ ] WalletActionModal.svelte — Success state
- [ ] WalletActionModal.svelte — Error state (form preserved)
- [ ] WalletActionModal.svelte — Validation errors
- [ ] WalletActionModal.svelte — High-value warning
- [ ] CustomerDetail.svelte — "+ Add" button on Cash row
- [ ] CustomerDetail.svelte — "+ Add" button on Points row
- [ ] CustomerDetail.svelte — "..." overflow menu with Remove/Expire
- [ ] CustomerDetail.svelte — Zero-balance overflow disabled
- [ ] Check `list_components` before building any UI element
- [ ] Run svelte-autofixer on all .svelte files

## Phase 8: Integration
- [ ] Export types and functions from `customers/index.ts`
- [ ] Backend routes registered with admin auth middleware
- [ ] Customer detail refresh after successful action
- [ ] End-to-end: Add Cash
- [ ] End-to-end: Add Points
- [ ] End-to-end: Remove Cash
- [ ] End-to-end: Remove Points
- [ ] End-to-end: Expire Cash
- [ ] End-to-end: Expire Points
- [ ] End-to-end: Validation errors
- [ ] End-to-end: Over-balance Remove error
- [ ] End-to-end: Tier impact preview (Points Remove/Expire)
- [ ] `npx svelte-check --threshold error` passes
- [ ] `cargo check` passes
- [ ] `cargo test` passes
