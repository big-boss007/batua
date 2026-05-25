# Implementation Checklist

## API Integration
- [x] Verify `claimGiftCard` exists in gift-cards remote (sends only `{ code }` — insufficient)
- [x] Add `claimGiftCardForCustomer` to storefront remote with `{ code, customer_id }`
- [x] Export from storefront barrel

## GiftCardStatus Component
- [x] Add `customerId` and `onClaimed` props
- [x] Add `claiming`, `claimSuccess`, `claimError` component state
- [x] Render claim button for active unclaimed cards (when customerId is not null)
- [x] Render success state after claim (checkmark + amount + "Back to Rewards")
- [x] Render "Already in your wallet" for claimed cards
- [x] Hide claim button for expired cards
- [x] Style matches approved design preview

## Check Page
- [x] Read `customerPhone` store
- [x] Look up customer ID via `lookupCustomer`
- [x] Pass `customerId` to GiftCardStatus
- [x] Handle `onClaimed` callback (goto '../')

## Verification
- [x] Active unclaimed card → claim button visible → click → success state
- [x] Already claimed card → no button, "Already in your wallet"
- [x] Not logged in → no claim button shown
- [x] DB updated: `is_claimed = true` after claim
- [x] Zero console errors
- [x] svelte-check passes with 0 errors
