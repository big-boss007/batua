# Phase 8: Integration

## Objective

Wire the earn multiplier into the backend earn flow and connect all frontend pieces.

---

## Backend: Earn Multiplier Wiring

### Location: `src/services/earn/helpers.rs` → `do_process_earn()`

### Current flow:
```
Order → build context → evaluate_rules → create ledger entries
```

### New flow:
```
Order → build context → evaluate_rules → query multipliers → apply highest → create ledger entries
```

### Implementation:

After `evaluate_rules()` returns `Vec<EvaluationResult>`, before creating ledger entries:

1. Query membership status: `get_membership_status(pool, merchant_id, customer_id)`
2. Query loyalty tier: `get_customer_tier(pool, merchant_id, customer_id)` (from loyalty service)
3. Determine effective multiplier:
   ```rust
   let membership_mult = membership_status
       .and_then(|s| if s.is_active { s.plan.map(|p| p.earn_rate_multiplier) } else { None })
       .unwrap_or(1.0);

   let tier_mult = customer_tier
       .map(|t| t.earn_rate_multiplier)
       .unwrap_or(1.0);

   let effective_mult = membership_mult.max(tier_mult);
   ```
4. Apply to each evaluation result:
   ```rust
   if effective_mult > 1.0 {
       for result in &mut eval_results {
           result.earning_unit *= effective_mult;
           result.currency_equivalent *= effective_mult;
       }
   }
   ```

### Backend: List Subscribers Endpoint

Add to `src/services/earn/`:
- **storage.rs**: `list_memberships_by_merchant(pool, merchant_id)` query
- **handler.rs**: `list_subscribers()` handler
- **mod.rs**: `GET /earn/memberships/subscribers/{merchant_id}` route

---

## Frontend: Wire Everything Together

### Admin page data flow:
1. On mount: fetch plans + subscribers using merchant from store
2. Plans section: CRUD operations update plan list
3. Subscribers section: assign/cancel update subscriber list
4. Use existing `lookupCustomer` from storefront module for customer search in AssignForm

### Storefront data flow:
1. In `loadCustomerData()`: add `getMembershipStatus()` to the `Promise.all`
2. Store result in `membershipStatus: MembershipStatus | null`
3. Pass to ProfileBar as props

## Tasks

- [ ] Wire multiplier into `do_process_earn()` in earn/helpers.rs
- [ ] Add `list_subscribers` endpoint to earn service
- [ ] Import loyalty tier query into earn helpers (cross-service call)
- [ ] Wire admin page data fetching and mutations
- [ ] Wire storefront membership status fetch
- [ ] Test earn flow with active membership → verify multiplied cashback
