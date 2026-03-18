# Gift Cards API

## Issue Gift Card

**Method:** POST
**Path:** `/gift-cards/issue`

Issues a new gift card. Creates a bearer wallet and a corresponding ledger entry in the `gift_card` bucket.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant issuing the gift card |
| amount | number | yes | - | Gift card value |
| expires_at | string | no | null | ISO 8601 expiration date |
| payment_reference | string | no | null | Payment reference for the purchase |
| actor_type | string | yes | - | Who issued it: "system", "human", "automation", "migration" |
| actor_id | string | no | null | Identifier of the issuer |

### Example Request

```bash
curl -X POST http://localhost:3000/gift-cards/issue \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "amount": 500.0,
    "expires_at": "2027-03-18T23:59:59Z",
    "payment_reference": "razorpay_pay_12345",
    "actor_type": "human",
    "actor_id": "admin_ravi"
  }'
```

### Response

**Status:** 201 Created

```json
{
  "id": "gc12a3b4-c5d6-7890-ef12-345678901234",
  "code": "DESI-ABCD-1234",
  "initial_amount": 500.0,
  "current_amount": 500.0,
  "is_claimed": false,
  "is_active": true,
  "expires_at": "2027-03-18T23:59:59Z",
  "created_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields or invalid amount |
| 500 | Database error |

---

## Bulk Issue Gift Cards

**Method:** POST
**Path:** `/gift-cards/bulk-issue`

Issues multiple gift cards in a single batch.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| batch_id | uuid | yes | - | Client-generated batch ID for tracking |
| cards | array | yes | - | Array of card items to issue |

**Card item:**

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| amount | number | yes | - | Gift card value |
| recipient_phone | string | no | null | Recipient phone for notification |
| recipient_email | string | no | null | Recipient email for notification |

### Example Request

```bash
curl -X POST http://localhost:3000/gift-cards/bulk-issue \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "batch_id": "ba12a3b4-c5d6-7890-ef12-345678901234",
    "cards": [
      {
        "amount": 500.0,
        "recipient_phone": "+919876543210",
        "recipient_email": "priya.sharma@gmail.com"
      },
      {
        "amount": 1000.0,
        "recipient_phone": "+919876543211"
      },
      {
        "amount": 250.0
      }
    ]
  }'
```

### Response

**Status:** 201 Created

```json
{
  "batch_id": "ba12a3b4-c5d6-7890-ef12-345678901234",
  "total_issued": 3,
  "total_skipped": 0,
  "cards": [
    {
      "id": "gc12a3b4-c5d6-7890-ef12-345678901234",
      "code": "DESI-ABCD-1234",
      "initial_amount": 500.0,
      "current_amount": 500.0,
      "is_claimed": false,
      "is_active": true,
      "expires_at": null,
      "created_at": "2026-03-18T10:00:00Z"
    },
    {
      "id": "gc23b4c5-d6e7-8901-f234-567890123456",
      "code": "DESI-EFGH-5678",
      "initial_amount": 1000.0,
      "current_amount": 1000.0,
      "is_claimed": false,
      "is_active": true,
      "expires_at": null,
      "created_at": "2026-03-18T10:00:00Z"
    },
    {
      "id": "gc34c5d6-e7f8-9012-a345-678901234567",
      "code": "DESI-IJKL-9012",
      "initial_amount": 250.0,
      "current_amount": 250.0,
      "is_claimed": false,
      "is_active": true,
      "expires_at": null,
      "created_at": "2026-03-18T10:00:00Z"
    }
  ]
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields or empty cards array |
| 500 | Database error |

---

## Claim Gift Card

**Method:** POST
**Path:** `/gift-cards/claim`

Claims a gift card for a customer. Transfers the gift card balance from the bearer wallet to the customer's wallet.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| code | string | yes | - | Gift card code |
| customer_id | uuid | yes | - | Customer claiming the card |

### Example Request

```bash
curl -X POST http://localhost:3000/gift-cards/claim \
  -H "Content-Type: application/json" \
  -d '{
    "code": "DESI-ABCD-1234",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012"
  }'
```

### Response

```json
{
  "id": "gc12a3b4-c5d6-7890-ef12-345678901234",
  "code": "DESI-ABCD-1234",
  "initial_amount": 500.0,
  "current_amount": 500.0,
  "is_claimed": true,
  "is_active": true,
  "expires_at": "2027-03-18T23:59:59Z",
  "created_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 404 | Gift card code not found |
| 409 | Gift card already claimed |
| 400 | Gift card expired or inactive |
| 500 | Database error |

---

## Redeem Gift Card

**Method:** POST
**Path:** `/gift-cards/redeem`

Redeems a specific amount from a gift card for an order.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| code | string | yes | - | Gift card code |
| amount | number | yes | - | Amount to redeem |
| order_id | string | yes | - | Order the redemption applies to |

### Example Request

```bash
curl -X POST http://localhost:3000/gift-cards/redeem \
  -H "Content-Type: application/json" \
  -d '{
    "code": "DESI-ABCD-1234",
    "amount": 200.0,
    "order_id": "shopify_order_5010"
  }'
```

### Response

```json
{
  "id": "gc12a3b4-c5d6-7890-ef12-345678901234",
  "code": "DESI-ABCD-1234",
  "initial_amount": 500.0,
  "current_amount": 300.0,
  "is_claimed": true,
  "is_active": true,
  "expires_at": "2027-03-18T23:59:59Z",
  "created_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields, insufficient balance, or card inactive |
| 404 | Gift card code not found |
| 400 | Amount exceeds current balance |
| 500 | Database error |

---

## Get Gift Card by Code

**Method:** GET
**Path:** `/gift-cards/{code}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| code | string | Gift card code |

### Example Request

```bash
curl http://localhost:3000/gift-cards/DESI-ABCD-1234
```

### Response

```json
{
  "id": "gc12a3b4-c5d6-7890-ef12-345678901234",
  "code": "DESI-ABCD-1234",
  "initial_amount": 500.0,
  "current_amount": 300.0,
  "is_claimed": true,
  "is_active": true,
  "expires_at": "2027-03-18T23:59:59Z",
  "created_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Gift card not found |
| 500 | Database error |

---

## List Gift Cards for Merchant

**Method:** GET
**Path:** `/gift-cards/merchant/{merchant_id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| page | integer | no | 1 | Page number (minimum 1) |
| limit | integer | no | 20 | Items per page (1-100) |

### Example Request

```bash
curl "http://localhost:3000/gift-cards/merchant/a1b2c3d4-e5f6-7890-abcd-ef1234567890?page=1&limit=10"
```

### Response

```json
[
  {
    "id": "gc12a3b4-c5d6-7890-ef12-345678901234",
    "code": "DESI-ABCD-1234",
    "initial_amount": 500.0,
    "current_amount": 300.0,
    "is_claimed": true,
    "is_active": true,
    "expires_at": "2027-03-18T23:59:59Z",
    "created_at": "2026-03-18T10:00:00Z"
  },
  {
    "id": "gc23b4c5-d6e7-8901-f234-567890123456",
    "code": "DESI-EFGH-5678",
    "initial_amount": 1000.0,
    "current_amount": 1000.0,
    "is_claimed": false,
    "is_active": true,
    "expires_at": null,
    "created_at": "2026-03-18T10:00:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |
