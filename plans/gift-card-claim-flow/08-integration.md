# Integration

## Route: `/s/{slug}/gift-cards/check`

1. Page loads → reads `customerPhone` from sessionStorage store
2. If phone exists → looks up customer via `lookupCustomer(phone)` → gets customer ID
3. Passes `customerId` to `GiftCardStatus` component
4. If no phone (not logged in) → passes `customerId={null}` → claim button hidden

## Navigation After Claim

"Back to Rewards" button uses `goto('../')` to navigate to `/s/{slug}` main storefront page.

## Testing Plan

1. Login at `/s/desidrapes` with a valid phone
2. Navigate to gift card check
3. Enter an active unclaimed card code
4. Verify "Claim ₹X to Wallet" button appears
5. Click claim → verify success state shows
6. Navigate back → verify balance updated
7. Check same code again → verify "Already in your wallet" shows
8. Try an expired card → verify no claim button
