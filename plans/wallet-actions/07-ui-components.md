# Phase 7: UI Components

## Objective

Build the WalletActionModal component and modify CustomerDetail for entry points. This is the largest phase.

## Tasks

### 1. WalletActionModal.svelte

Single component with all 3 flows, following the design in `docs/wallet-actions-design.html`.

**Structure:**
```
Modal (from @juspay/svelte-ui-components)
  ├── Header: "Wallet Action" + close button
  ├── Body (step === 'form'):
  │   ├── Action Tabs: [Add] [Remove] [Expire]
  │   ├── Customer Banner: avatar, name, phone, current balance
  │   ├── Bucket Selector: radio (Add/Remove) or checkbox (Expire)
  │   ├── Amount + Expiry row (Add/Remove only)
  │   ├── Reason pills + textarea
  │   ├── Reference input
  │   ├── Notify toggle (Add only)
  │   ├── Preview box (green/red/amber)
  │   ├── Tier impact (points Remove/Expire only)
  │   └── Validation errors (inline)
  ├── Body (step === 'confirm'):
  │   ├── Icon + title + subtitle
  │   ├── Summary table
  │   ├── Type-to-confirm (Remove: amount, Expire: "EXPIRE")
  │   └── Confirm/Go Back buttons
  ├── Body (step === 'loading'):
  │   └── Same as confirm but disabled with spinner
  ├── Body (step === 'success'):
  │   ├── Check icon + amount + subtitle
  │   ├── Result details (txn ID, new balance)
  │   └── Done button
  ├── Body (step === 'error'):
  │   ├── Error banner (dismissible)
  │   └── Form preserved with Retry button
  └── Footer: Cancel + Action button (color matches action)
```

**Component library usage (MUST check list_components first):**
- Modal, Button, Input, Toggle, Avatar, Pill from `@juspay/svelte-ui-components`
- Check `get_component_docs` for exact prop signatures

**Color system:**
- Add: `--color-success` (green backgrounds, buttons)
- Remove: `--color-error` (red backgrounds, buttons)
- Expire: `--color-warning` (amber backgrounds, buttons)

**Key interactions:**
- Tab switching resets form fields but preserves customer context
- Bucket selector filters by action + unit
- Amount input validates on change (not just submit)
- Preview updates live as amount changes
- High-value warning appears above a merchant-configurable threshold
- Stale balance detection on submit (re-fetch and compare)

### 2. CustomerDetail.svelte Modifications

**Entry points on wallet rows:**

Points row:
```
[dot] Points  [845 ★]  [+ Add]  [···]
                                  ├── Remove
                                  └── Expire
```

Cash row:
```
[dot] Cash  [₹477]  [+ Add]  [···]
                               ├── Remove
                               └── Expire
```

**Changes needed:**
1. Add `showWalletAction` state variable
2. Add `walletActionType` and `walletActionUnit` state variables
3. Add "+ Add" button to each wallet row
4. Add "..." overflow button with dropdown menu
5. Render `WalletActionModal` when `showWalletAction` is true
6. On success callback: emit event to parent to refresh customer detail

**Zero-balance handling:**
- "+ Add" always visible
- "..." menu disabled when balance is 0 for that unit

### 3. Svelte Autofixer

Run `svelte-autofixer` on all modified Svelte files before finishing.

## Outputs

- WalletActionModal.svelte — complete component
- CustomerDetail.svelte — modified with entry points
- All states from the design mockup implemented

## Validation

- `npx svelte-check --threshold error` passes
- Visual verification in browser against design mockup
- Test all 6 operations end-to-end
- Test validation errors, loading, success, error states
- Test tab switching preserves/resets state correctly
