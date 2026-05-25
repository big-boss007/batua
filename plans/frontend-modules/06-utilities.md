# Phase 6: Utilities

Status: COMPLETED

## foundation/utils.ts

Shared formatting utilities used across the entire frontend.

### formatCurrencyINR(amount: number): string
Formats a number as Indian Rupee currency using `Intl.NumberFormat('en-IN', { style: 'currency', currency: 'INR' })`. Two decimal places. Example: `formatCurrencyINR(1250.5)` -> `"$1,250.50"` (INR symbol).

### formatDate(iso: string): string
Formats ISO date string to `en-IN` short date. Uses `Intl.DateTimeFormat('en-IN', { year: 'numeric', month: 'short', day: 'numeric' })`. Example: `"2025-03-15T..."` -> `"15 Mar 2025"`.

### formatDateTime(iso: string): string
Formats ISO date string to `en-IN` with time. Adds `hour: '2-digit', minute: '2-digit'` to the date format. Example: `"2025-03-15T14:30:..."` -> `"15 Mar 2025, 02:30 pm"`.

### normalizePhoneE164(phone: string, countryCode?: string): string
Normalizes phone numbers to E.164 format. Default country code: `'91'` (India).
- Already has country code: `"919876543210"` -> `"+919876543210"`
- Starts with 0: `"09876543210"` -> `"+919876543210"`
- Local number: `"9876543210"` -> `"+919876543210"`

Strips all non-digit characters before processing.

## admin/utils.ts

Merchant admin formatting and localStorage helpers.

### formatMetricValue(value: number, metricType: MetricType): string
Formats a metric value based on its type:
- `'currency'` -> INR currency format (same as `formatCurrencyINR`)
- `'percentage'` -> `"XX.X%"` (1 decimal place)
- `'number'` -> `en-IN` locale number format with grouping

Uses separate `Intl.NumberFormat` instances for number and currency.

### getCurrentMerchantId(): string | null
Reads `batua_merchant_id` from localStorage. Returns `null` during SSR or if unavailable.

### setCurrentMerchantId(id: string): void
Writes merchant ID to localStorage key `batua_merchant_id`. No-op during SSR.

### MERCHANT_ID_KEY = 'batua_merchant_id'
Constant for the localStorage key. Exported but only used within admin module.

## transactions/utils.ts

Formatting helpers for transaction display with color-coded hints.

### BUCKET_LABELS: Record<string, string>
Maps bucket type slugs to display labels:
- `earned_credit` -> `"Earned Credit"`
- `cod_pending` -> `"COD Pending"`
- `gift_card` -> `"Gift Card"`
- `customer_funded` -> `"Customer Funded"`
- `referral_reward` -> `"Referral Reward"`
- `goodwill_credit` -> `"Goodwill Credit"`
- `membership_benefit` -> `"Membership Benefit"`
- `refund_credit` -> `"Refund Credit"`

### formatBucketType(bucketType: string): string
Looks up `BUCKET_LABELS`, falls back to title-casing the snake_case string.

### formatMovementType(movementType: string): MovementHint
Returns `{ label, color }` for movement types:
- `"in"` -> `{ label: 'In', color: 'var(--color-success)' }`
- `"out"` -> `{ label: 'Out', color: 'var(--color-error)' }`
- `"held"` -> `{ label: 'Held', color: 'var(--color-warning)' }`
- `"across"` -> `{ label: 'Across', color: 'var(--color-info)' }`

Case-insensitive lookup. Falls back to `{ label: formatted_string, color: 'var(--color-text-muted)' }`.

### formatState(state: string): StateHint
Returns `{ label, color }` for entry states:
- `completed` -> success (green)
- `pending` -> warning (amber)
- `failed` -> error (red)
- `cancelled` -> muted
- `reversed` -> info (blue)
- `processing` -> info (blue)
- `approved` -> success (green)
- `rejected` -> error (red)

## customers/utils.ts

Loyalty tier formatting utilities.

### getTierColor(rank: number): string
Maps tier rank to CSS custom property:
- Rank 1 -> `var(--color-text-muted)` (basic)
- Rank 2 -> `var(--color-info)` (blue)
- Rank 3 -> `var(--color-warning)` (amber)
- Rank 4 -> `var(--color-success)` (green)
- Rank 5 -> `var(--color-primary)` (purple)

### formatMultiplier(multiplier: number): string
Formats earn rate multiplier. Example: `formatMultiplier(2)` -> `"2x"`.

### sortTiersByRank(tiers: Array<LoyaltyTier>): Array<LoyaltyTier>
Returns a new array sorted by `rank` ascending. Does not mutate the input.

### formatMovementType(movement: string): string
Simple title-case conversion: replaces underscores with spaces, capitalizes each word.

### formatBucketType(bucket: string): string
Same `BUCKET_LABELS` map as transactions/utils (duplicated). Falls back to title-casing.

## storefront/utils.ts

Customer-facing utility functions.

### generateWhatsAppShareUrl(text: string): string
Generates a WhatsApp share URL: `https://wa.me/?text={encodedText}`.

### copyToClipboard(text: string): Promise<boolean>
Copies text to clipboard using `navigator.clipboard.writeText`. Returns `true` on success, `false` on failure.

### formatBucketLabel(type: string): string
Same bucket label mapping as transactions and customers utils (third instance). Falls back to title-casing.

### formatMovementLabel(type: string): string
Maps movement types to customer-friendly labels:
- `"In"` -> `"Earned"`
- `"Out"` -> `"Redeemed"`
- `"Held"` -> `"Held"`
- `"Released"` -> `"Released"`
- `"Expired"` -> `"Expired"`
- `"Across"` -> `"Transferred"`

### getMovementPrefix(type: string): string
Returns the sign prefix for a movement type:
- `"In"`, `"Released"` -> `"+"`
- `"Out"`, `"Expired"` -> `"-"`
- `"Held"`, `"Across"` -> `""`

### maskPhone(phone: string): string
Masks a phone number showing only the last 4 digits:
- `"+919876543210"` -> `"+91******3210"`
- `"9876543210"` -> `"******3210"`

## Shared Constants Across Modules

The `BUCKET_LABELS` map is duplicated in three places:
1. `transactions/utils.ts` (as `BUCKET_LABELS`)
2. `customers/utils.ts` (as `BUCKET_LABELS`)
3. `storefront/utils.ts` (as `bucketLabels`)

All three contain the same 8 bucket type mappings. This duplication is intentional per the module isolation principle -- each module owns its own utility surface.
