# Phase 3: Type Definitions

## Objective

Define frontend TypeScript types mirroring the backend.

## Types to create in `memberships/types.ts`

```typescript
type MembershipPlan = {
  id: string
  merchant_id: string
  name: string
  plan_type: string        // "annual" | "monthly"
  price: number
  earn_rate_multiplier: number
  benefits: Record<string, unknown> | null
  is_active: boolean
  created_at: string
}

type CustomerMembership = {
  id: string
  merchant_id: string
  customer_id: string
  plan_id: string
  status: string           // "active" | "expired" | "cancelled"
  started_at: string
  expires_at: string
  renewed_count: number
  cancelled_at: string | null
  created_at: string
}

type MembershipStatus = {
  membership: CustomerMembership | null
  plan: MembershipPlan | null
  is_active: boolean
  days_remaining: number
}

type CreatePlanRequest = {
  merchant_id: string
  name: string
  plan_type: string
  price: number
  earn_rate_multiplier: number | null
  benefits: Record<string, unknown> | null
}

type SubscribeRequest = {
  merchant_id: string
  customer_id: string
  plan_id: string
}
```

## Tasks

- [ ] Create `types.ts` with above types
- [ ] Export from barrel `index.ts`
