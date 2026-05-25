# State Management

## Component State (in GiftCardStatus)

| State | Type | Purpose |
|-------|------|---------|
| `claiming` | `boolean` | Loading state while claim API is in-flight |
| `claimSuccess` | `boolean` | Whether claim succeeded — switches to success view |
| `claimError` | `string \| null` | Error message if claim fails |

## Existing State Used

- `customerPhone` store — read to determine if customer is logged in
- Customer ID — resolved via `lookupCustomer(phone)` in the check page, passed as prop to GiftCardStatus

## No URL or Store State Changes

All claim state is component-local — dies when navigating away. No URL params or persistent stores needed.
