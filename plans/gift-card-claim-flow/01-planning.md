# Planning

## Architecture Decision

Option B was chosen: customer must be logged in on the storefront (`/s/{slug}`) before navigating to gift card check. The `customerPhone` store provides the phone number, which is used to look up the customer ID for the claim API call.

## Flow

1. Customer logs in at `/s/{slug}` with phone number
2. Navigates to gift card check via bottom action or quick link
3. Enters gift card code → sees card details
4. If card is active + unclaimed → "Claim ₹X to Wallet" button appears
5. Click claim → `POST /gift-cards/claim` with `{ code, customer_id }`
6. Success → show success state with amount, "Back to Rewards" button
7. If card already claimed → show "Already in your wallet" text
8. If card expired → no action available

## Files to Modify

1. `frontend/src/lib/client/modules/storefront/ui/GiftCardStatus.svelte` — add claim button, success state, claimed state
2. `frontend/src/routes/s/[slug]/gift-cards/check/+page.svelte` — pass customer context to GiftCardStatus, handle claim callback
3. `frontend/src/lib/client/modules/storefront/remote.ts` — add `claimGiftCard` function if not already exported

## API Contract

`POST /gift-cards/claim`
```json
{
  "code": "BRZE-XXXX-XXXX-S25M",
  "customer_id": "uuid"
}
```
Response: updated GiftCard object with `is_claimed: true`
