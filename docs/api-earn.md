# Earn API

## Process Earn

**Method:** POST
**Path:** `/earn/process`

Processes an event through the earn pipeline. Evaluates all active rules against the event, creates ledger entries for matched rules, and handles COD-specific flows (held credits).

This is the primary orchestration endpoint that ties together events, rules, wallets, and the ledger.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| event_id | uuid | yes | - | ID of a previously ingested event to process |

### Example Request

```bash
curl -X POST http://localhost:3000/earn/process \
  -H "Content-Type: application/json" \
  -d '{
    "event_id": "ev12a3b4-c5d6-7890-ef12-345678901234"
  }'
```

### Response

```json
{
  "event_id": "ev12a3b4-c5d6-7890-ef12-345678901234",
  "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
  "entries_created": [
    {
      "ledger_entry_id": "le12a3b4-c5d6-7890-ef12-345678901234",
      "bucket_type": "earned_credit",
      "earning_unit": 74.95,
      "currency_equivalent": 74.95,
      "movement_type": "in"
    }
  ],
  "is_cod": false
}
```

### Response (COD Order)

When the event is a COD order, credits are held (not immediately spendable) until delivery confirmation.

```json
{
  "event_id": "ev23b4c5-d6e7-8901-f234-567890123456",
  "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
  "entries_created": [
    {
      "ledger_entry_id": "le23b4c5-d6e7-8901-f234-567890123456",
      "bucket_type": "cod_pending",
      "earning_unit": 74.95,
      "currency_equivalent": 74.95,
      "movement_type": "held"
    }
  ],
  "is_cod": true
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing event_id |
| 404 | Event not found |
| 404 | No customer found for the event (cannot resolve identity) |
| 500 | Database or rule evaluation error |

---

## Manual Credit

**Method:** POST
**Path:** `/earn/manual-credit`

Credits a customer's wallet directly, bypassing the event/rule pipeline. Used for goodwill credits, compensation, or admin adjustments.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| customer_id | uuid | yes | - | Customer to credit |
| amount | number | yes | - | Amount to credit (in currency) |
| bucket_type | string | yes | - | Bucket to credit (e.g. "goodwill_credit", "earned_credit") |
| reason | string | yes | - | Audit reason for the credit |
| actor_id | string | yes | - | ID of the person or system making the credit |

### Example Request

```bash
curl -X POST http://localhost:3000/earn/manual-credit \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "amount": 200.0,
    "bucket_type": "goodwill_credit",
    "reason": "Compensation for delayed delivery on order #1042",
    "actor_id": "admin_ravi"
  }'
```

### Response

**Status:** 201 Created

```json
{
  "ledger_entry_id": "le34c5d6-e7f8-9012-a345-678901234567",
  "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
  "amount": 200.0
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields or invalid bucket_type |
| 404 | Customer or wallet not found |
| 500 | Database error |
