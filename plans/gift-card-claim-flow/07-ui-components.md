# UI Components

## GiftCardStatus.svelte — Modified

### New Props

| Prop | Type | Purpose |
|------|------|---------|
| `customerId` | `string \| null` | Customer ID for claim API. null = not logged in, hide claim button |
| `onClaimed` | `(() => void) \| null` | Callback after successful claim |

### New States (4 total, matching approved design)

1. **Active (unclaimed)** — card details + green "Claim ₹X to Wallet" button + hint
2. **Claim Success** — checkmark icon + "+₹X" amount + "Back to Rewards" button
3. **Already Claimed** — card details + "Claimed" pill + "Already in your wallet" text
4. **Expired** — card details + red "Expired" pill, no button

### Styling

Match `docs/gift-card-claim-preview.html` exactly:
- Claim button: `background: #4ade80`, `color: #000`, `border-radius: 10px`, full width
- Success icon: 56px circle with checkmark, green tint background
- Success amount: 32px bold green
- Hint text: 11px muted, centered below button

## Check Page (+page.svelte) — Modified

- Read `customerPhone` store to get phone
- Look up customer ID via `lookupCustomer`
- Pass `customerId` prop to GiftCardStatus
- Handle `onClaimed` callback (navigate back to storefront)
