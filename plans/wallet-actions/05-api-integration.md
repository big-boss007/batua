# Phase 5: API Integration

## Objective

Create backend endpoints for Remove and Expire, and frontend API client functions for all 3 operations.

## Tasks

### Backend: POST /admin/debit

**Handler** (`admin/handler.rs`):
1. Parse `AdminDebitRequest` from JSON body
2. Validate: amount > 0, bucket exists, bucket has sufficient balance
3. Call `process_debit` helper
4. Return `AdminDebitResult`

**Storage** (`admin/storage.rs` or `ledger/storage.rs`):
1. Begin transaction
2. Find active ledger entries for the customer+bucket, ordered by created_at ASC (FIFO)
3. Create new ledger entry with `movement_type: Out`, `state: Active`
4. Decrease bucket spendable balance
5. Commit transaction
6. Return result with new balance

**Route**: Add to admin router in `handler.rs` or `mod.rs`

### Backend: POST /admin/force-expire

**Handler** (`admin/handler.rs`):
1. Parse `AdminExpireRequest` from JSON body
2. Validate: bucket_types non-empty, each bucket has balance > 0
3. Call `process_force_expire` helper
4. Return `AdminExpireResult`

**Storage**:
1. Begin transaction
2. For each bucket_type in request:
   - Find all active ledger entries for customer+bucket
   - Set `state: Expired` and `expires_at: now()` on each
   - Sum total expired amount
3. Commit transaction
4. Return total entries expired, amount expired, new balance

### Frontend: API Client (customers/remote.ts)

```typescript
async function addCredit(req: AddRequest): Promise<WalletActionResult>
  // POST /admin/bulk-credit (existing endpoint, wrap single customer)

async function removeBalance(req: RemoveRequest): Promise<WalletActionResult>
  // POST /admin/debit (new endpoint)

async function expireBalance(req: ExpireRequest): Promise<WalletActionResult>
  // POST /admin/force-expire (new endpoint)
```

### Idempotency

- Add: existing bulk-credit already has idempotency via `idempotency_key`
- Remove: generate client-side idempotency key from `customer_id + bucket + amount + timestamp`
- Expire: generate from `customer_id + bucket_types.join() + timestamp`

## Outputs

- Two new backend endpoints functional
- Three frontend API functions callable
- Idempotency keys prevent double-processing

## Validation

- `cargo check` passes
- `cargo test` passes
- Manual curl test of both new endpoints
- Frontend functions return expected types
