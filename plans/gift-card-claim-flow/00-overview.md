# Gift Card Claim Flow

## Status: IN PROGRESS

## Goal

Add a "Claim to Wallet" button on the storefront gift card check page (`/s/{slug}/gift-cards/check`) so logged-in customers can claim unclaimed gift cards to their wallet's `gift_card` bucket.

## Scope

**In scope:**
- Claim button on GiftCardStatus component for active unclaimed cards
- Success state after claim (checkmark + amount + "Back to Rewards")
- Already-claimed state (no button, "Already in your wallet" text)
- Expired state (no button)
- Frontend calls `POST /gift-cards/claim` with code + customer_id
- Customer must be identified (phone stored in sessionStorage from storefront login)

**Out of scope:**
- Backend changes (claim endpoint already exists)
- Claim from anonymous/unauthenticated state (Option B chosen — requires storefront login first)
- Gift card redemption at checkout (separate flow)

## Success Criteria

- [ ] Active unclaimed card shows green "Claim ₹X to Wallet" button
- [ ] Clicking claim calls API and shows success state with amount
- [ ] Already-claimed cards show "Already in your wallet" text, no button
- [ ] Expired cards show no button
- [ ] Customer phone/ID passed from storefront session
- [ ] Matches approved design preview (`docs/gift-card-claim-preview.html`)

## Dependencies

- Backend: `POST /gift-cards/claim` — already exists
- Storefront session: `customerPhone` store in storefront module — already exists
- Customer lookup: `lookupCustomer` in storefront remote — already exists
