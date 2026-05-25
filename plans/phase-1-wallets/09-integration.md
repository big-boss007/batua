# Phase 1: Integration -- COMPLETED

## How the Four Services Work Together

### The Earn -> Ledger -> Wallet Pipeline

All value flows through a single pipeline:

1. **Earn** service determines what to credit (amount, bucket, cause).
2. **Ledger** service records the immutable entry with idempotency.
3. **Wallet** service provides the wallet_id that scopes all entries.

The earn service is the primary producer of ledger entries. It never writes to the `wallets` table directly -- it calls `wallets::storage::get_or_create_wallet` to resolve the target wallet, then writes to `ledger_entries` via `ledger::storage::create_entry`.

### COD Integration with Earn

When the earn service processes an order that is COD:

1. **Earn** creates a `Held` movement in `CodPending` bucket (not `In` to `EarnedCredit`).
2. **Earn** also calls `cod::storage::create_cod_order` to track the hold.
3. Later, when a delivery webhook arrives:
   - **COD** service's `process_delivery` creates an `Across` movement: Out from `CodPending` + In to `EarnedCredit`. The value is now spendable.
   - Or **COD** service's `process_rto` creates a single `Out` from `CodPending`. The value is destroyed.

This ensures COD cashback is never spendable until delivery is confirmed.

### Redemption Integration with Ledger and Wallet Policies

The redemption flow reads from multiple sources:

1. **Wallet** balance (via `ledger::storage::get_balance`) to determine what is available per bucket.
2. **Wallet policies** (via `redemption::storage::get_wallet_policies`) to determine per-bucket constraints (caps, step sizes, exclusions).
3. Creates `Out` entries in the ledger to debit value.
4. If the Shopify discount fails, creates compensating `In` entries to restore value.

### Cross-Service Dependencies

```
earn::helpers::process_earn
  -> events::storage::get_event              (reads event)
  -> identity::storage::resolve_or_create    (resolves customer)
  -> wallets::storage::get_or_create_wallet  (resolves wallet)
  -> rules::helpers::evaluate_rules          (evaluates earn rules)
  -> ledger::storage::create_entry           (creates ledger entry)
  -> cod::storage::create_cod_order          (if COD)
  -> earn::storage::update_order_stats       (updates stats)

redemption::helpers::execute_redemption
  -> wallets::storage::get_wallet            (reads wallet)
  -> ledger::storage::get_balance            (reads balance)
  -> redemption::storage::get_wallet_policies (reads policies)
  -> ledger::storage::create_entry           (creates debit entries)

cod::helpers::process_delivery
  -> cod::storage::get_cod_order_by_order_id (reads COD order)
  -> ledger::storage::get_entries            (finds held entry)
  -> ledger::storage::create_across_movement (out+in pair)
  -> cod::storage::update_cod_state          (updates COD state)
```

### Data Flow Diagram

```
Shopify Webhook
      |
      v
  [Events Service]
      |
      v
  [Earn Service] ---> [Identity Service]
      |                    (resolve customer)
      |
      +---> [Wallets Service]
      |        (get_or_create)
      |
      +---> [Rules Service]
      |        (evaluate)
      |
      +---> [Ledger Service]
      |        (create entries)
      |
      +---> [COD Service]        (if COD order)
               (track hold)

Delivery Webhook
      |
      v
  [COD Service] ---> [Ledger Service]
                        (across movement)

Checkout
      |
      v
  [Redemption Service] ---> [Ledger Service]
      |                        (balance, debit)
      +---> [Wallet Policies]
               (constraints)
```

### Idempotency Strategy by Service

| Service | Key Format | Deterministic? |
|---------|-----------|----------------|
| Earn (order) | `earn:{event_id}:{rule_snapshot_id}` | Yes -- replay safe |
| Earn (manual) | `manual:{merchant}:{customer}:{uuid}` | No -- each call unique |
| Earn (birthday) | `birthday:{sha256(merchant+customer+date)}` | Yes -- one per day |
| Earn (newsletter) | `newsletter:{sha256(merchant+customer)}` | Yes -- one ever |
| Earn (profile) | `profile_complete:{sha256(merchant+customer)}` | Yes -- one ever |
| Earn (milestone) | `milestone:{sha256(merchant+customer+milestone)}` | Yes -- one per milestone |
| Earn (streak) | `streak:{sha256(merchant+customer+config+window)}` | Yes -- one per window |
| Earn (spin) | `spin:{customer}:{uuid}` | No -- each spin unique |
| Redemption (debit) | `redemption-{id}-{bucket_type}` | Yes -- one per bucket per redemption |
| Redemption (comp) | `compensation-{id}-{bucket_type}` | Yes -- one per bucket per compensation |
| COD (delivery out) | `cod-delivery-out-{merchant}-{order}` | Yes -- one per order |
| COD (delivery in) | `cod-delivery-in-{merchant}-{order}` | Yes -- one per order |
| COD (rto) | `cod-rto-{merchant}-{order}` | Yes -- one per order |
| COD (prepaid) | `cod-prepaid-{merchant}-{customer}-{order}` | Yes -- one per order |
