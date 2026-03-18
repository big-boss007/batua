# Redemption API

## Initiate Redemption

**Method:** POST
**Path:** `/redemptions`

Initiates a redemption request to spend wallet credits on an order. The system validates the request against wallet policies (min redemption, step size, max per order, stackability) and creates debit ledger entries if eligible.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| wallet_id | uuid | yes | - | Wallet to redeem from |
| order_id | string | yes | - | External order ID (e.g. Shopify order ID) |
| order_amount | number | yes | - | Total order amount |
| payment_method | string | no | null | Payment method used for the order |
| requested_amount | number | yes | - | Amount the customer wants to redeem |
| discount_codes | string[] | yes | - | Discount codes applied to the order (for stackability check) |

### Example Request

```bash
curl -X POST http://localhost:3000/redemptions \
  -H "Content-Type: application/json" \
  -d '{
    "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
    "order_id": "shopify_order_5002",
    "order_amount": 2499.0,
    "payment_method": "razorpay",
    "requested_amount": 150.0,
    "discount_codes": []
  }'
```

### Response

**Status:** 201 Created

```json
{
  "redemption_id": "rd12a3b4-c5d6-7890-ef12-345678901234",
  "state": "Applied",
  "applied_amount": 150.0,
  "buckets_debited": [
    {
      "bucket_type": "EarnedCredit",
      "amount": 150.0,
      "entry_id": "le45d6e7-f8a9-0123-b456-789012345678"
    }
  ]
}
```

### Response (Partial Redemption)

When the requested amount exceeds what is available or policy-allowed:

```json
{
  "redemption_id": "rd12a3b4-c5d6-7890-ef12-345678901234",
  "state": "Applied",
  "applied_amount": 100.0,
  "buckets_debited": [
    {
      "bucket_type": "EarnedCredit",
      "amount": 75.0,
      "entry_id": "le45d6e7-f8a9-0123-b456-789012345678"
    },
    {
      "bucket_type": "GoodwillCredit",
      "amount": 25.0,
      "entry_id": "le56e7f8-a9b0-1234-c567-890123456789"
    }
  ]
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields, amount below minimum, or policy violation |
| 404 | Wallet not found |
| 409 | Duplicate redemption for the same order |
| 500 | Database error |

---

## Get Redemption

**Method:** GET
**Path:** `/redemptions/{id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Redemption ID |

### Example Request

```bash
curl http://localhost:3000/redemptions/rd12a3b4-c5d6-7890-ef12-345678901234
```

### Response

```json
{
  "id": "rd12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
  "requested_amount": 150.0,
  "eligible_amount": 150.0,
  "applied_amount": 150.0,
  "order_id": "shopify_order_5002",
  "order_amount": 2499.0,
  "payment_method": "razorpay",
  "state": "Applied",
  "debit_entry_id": "le45d6e7-f8a9-0123-b456-789012345678",
  "compensation_entry_id": null,
  "shopify_discount_id": null,
  "rejection_reason": null,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:01Z"
}
```

### Redemption States

| State | Description |
|-------|-------------|
| Initiated | Redemption request created |
| Validating | Checking wallet policies and balance |
| Rejected | Failed validation (see rejection_reason) |
| Committed | Debit entries created |
| Applied | Discount applied on the order |
| Failed | Post-commit failure |
| Compensated | Credits returned after failed order |
| Completed | Order fulfilled, redemption finalized |

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Redemption not found |
| 500 | Database error |

---

## Compensate Redemption

**Method:** POST
**Path:** `/redemptions/{id}/compensate`

Reverses a redemption by crediting the debited amount back to the wallet. Used when an order is cancelled or refunded after credits were applied.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Redemption ID to compensate |

### Example Request

```bash
curl -X POST http://localhost:3000/redemptions/rd12a3b4-c5d6-7890-ef12-345678901234/compensate
```

### Response

```json
{
  "id": "rd12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
  "requested_amount": 150.0,
  "eligible_amount": 150.0,
  "applied_amount": 150.0,
  "order_id": "shopify_order_5002",
  "order_amount": 2499.0,
  "payment_method": "razorpay",
  "state": "Compensated",
  "debit_entry_id": "le45d6e7-f8a9-0123-b456-789012345678",
  "compensation_entry_id": "le67f8a9-b0c1-2345-d678-901234567890",
  "shopify_discount_id": null,
  "rejection_reason": null,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T11:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Redemption is not in a compensable state |
| 404 | Redemption not found |
| 500 | Database error |

---

## Check Eligibility

**Method:** GET
**Path:** `/wallets/{wallet_id}/eligibility`

Checks how much a customer is eligible to redeem from their wallet for a given order, broken down by bucket.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| wallet_id | uuid | Wallet ID |

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| order_amount | number | yes | - | Order total amount |
| payment_method | string | no | null | Payment method for the order |

### Example Request

```bash
curl "http://localhost:3000/wallets/w1a2l3l4-e5t6-7890-abcd-ef1234567890/eligibility?order_amount=2499.0&payment_method=razorpay"
```

### Response

```json
{
  "total_eligible": 200.0,
  "buckets": [
    {
      "bucket_type": "EarnedCredit",
      "eligible_amount": 150.0,
      "constraints": {}
    },
    {
      "bucket_type": "GoodwillCredit",
      "eligible_amount": 50.0,
      "constraints": {}
    }
  ]
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing order_amount |
| 500 | Database error |
