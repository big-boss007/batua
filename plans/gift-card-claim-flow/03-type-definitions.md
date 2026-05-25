# Type Definitions

## No New Types Needed

The existing `GiftCardInfo` type in `storefront/types.ts` already has `is_claimed: boolean`, `is_active: boolean`, `current_amount: number`, `code: string` — all fields needed for the claim flow.

The claim API response reuses the existing `GiftCard` type from `gift-cards/types.ts`.

## Claim Request Shape

```typescript
{ code: string; customer_id: string }
```

This is constructed inline in the remote function, no separate type needed.
