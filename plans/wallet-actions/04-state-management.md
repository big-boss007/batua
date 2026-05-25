# Phase 4: State Management

## Objective

Define component-level state for the WalletActionModal. No Svelte stores needed — all state is local to the modal component using `$state` and `$derived` runes.

## Tasks

### Component State (inside WalletActionModal.svelte)

```
Props:
  - detail: CustomerDetail (the full customer detail object)
  - initialAction: WalletActionType ('add' | 'remove' | 'expire')
  - initialUnit: WalletUnitType ('cash' | 'points')
  - pointsRate: number (conversion rate, e.g. 0.1 for 10:1)
  - pointsIcon: string
  - onClose: () => void
  - onSuccess: () => void (triggers refresh of customer detail)

Local $state:
  - action: WalletActionType
  - step: WalletActionState ('form' | 'confirm' | 'loading' | 'success' | 'error')
  - selectedBucket: string | null
  - selectedBuckets: Set<string> (for expire multi-select)
  - amount: string (raw input, parsed to number for submission)
  - expiryDays: string
  - reasonCategory: string
  - reasonText: string
  - reference: string
  - notifyCustomer: boolean
  - errorMessage: string | null
  - result: WalletActionResult | null

Derived ($derived):
  - unit: WalletUnitType (from initialUnit, doesn't change)
  - wallet: WalletSummary | null
  - availableBuckets: BucketBalance[] (filtered by unit + action)
  - selectedBucketBalance: number (balance of selected bucket)
  - parsedAmount: number
  - isValid: boolean (all required fields filled, amount within range)
  - previewNewBalance: number
  - tierImpact: { willChange: boolean, from: string, to: string } | null
```

### Reason Categories (contextual per action)

```
Add:    ['Refund', 'Goodwill', 'Apology', 'Compensation', 'Promotional', 'Other']
Remove: ['Incorrect', 'Fraud/abuse', 'Chargeback', 'Dispute', 'Correction', 'Other']
Expire: ['Campaign ended', 'Policy change', 'Fraud/abuse', 'Account cleanup', 'Other']
```

### Bucket Filtering Logic

```
Add + Cash:    [RefundCredit, CustomerFunded]
Add + Points:  [GoodwillCredit, EarnedCredit]
Remove + Cash: buckets where !isPointsBucket && spendable > 0
Remove + Points: buckets where isPointsBucket && spendable > 0
Expire + Cash: same as Remove Cash
Expire + Points: same as Remove Points
```

## Outputs

- State shape fully defined before building the component
- Derived computations identified

## Validation

- State covers all 28 design states from the HTML mockup
- No `$effect` used (per codebase rules)
