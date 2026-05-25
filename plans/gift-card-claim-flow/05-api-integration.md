# API Integration

## Existing Endpoint

`POST /gift-cards/claim` — already exists in backend (`src/services/gift_cards/handler.rs`)

## Frontend Remote Function

Check if `claimGiftCard` already exists in `storefront/remote.ts`. If not, add:

```typescript
async function claimGiftCard(code: string, customerId: string): Promise<APIResult<GiftCardInfo>> {
  return apiCaller.post('/gift-cards/claim', { code, customer_id: customerId }, decodeGiftCardInfo);
}
```

Export from barrel if needed.

## Backend Request Shape

From `src/services/gift_cards/types.rs`:
```rust
pub struct ClaimGiftCardRequest {
    pub code: String,
    pub customer_id: Uuid,
}
```

Response: the full GiftCard object with `is_claimed: true`.
