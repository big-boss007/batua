# Phase 6: Utilities

## Objective

Add helper functions needed by the WalletActionModal component.

## Tasks

### Bucket Label Helpers (already exist in customers/utils.ts)

The existing `BUCKET_LABELS` and `formatBucketType` in `customers/utils.ts` already handle all bucket types. No new utility needed.

### New Utilities (add to customers/utils.ts if needed)

```typescript
function getAddBuckets(unit: WalletUnitType): Array<{ type: string; label: string }>
  // Returns the bucket options for Add action based on unit

function getActionReasonPills(action: WalletActionType): Array<string>
  // Returns contextual reason categories per action type

function formatTierImpact(
  currentTier: CustomerTierInfo | null,
  pointsChange: number,
  pointsRate: number
): { willChange: boolean; fromTier: string; toTier: string } | null
  // Calculates whether a point debit/expire will affect the customer's tier
```

### Validation Helpers

```typescript
function validateWalletAction(
  action: WalletActionType,
  amount: number,
  bucketBalance: number,
  reasonCategory: string,
  reasonText: string
): Array<{ field: string; message: string }>
  // Returns array of validation errors (empty = valid)
```

## Outputs

- Helper functions for bucket filtering, reason pills, tier impact, validation

## Validation

- Functions are pure (no side effects), easily testable
- Used by WalletActionModal component
