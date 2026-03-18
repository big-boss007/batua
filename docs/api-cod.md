# COD (Cash on Delivery) API

Manages the lifecycle of credits earned on COD orders. Credits are held in the `cod_pending` bucket until delivery is confirmed, then released to `earned_credit`. If the order is returned (RTO) or cancelled, the held credits are reversed.

## Delivery Webhook

**Method:** POST
**Path:** `/cod/webhook/delivery`

Receives delivery status updates from the logistics provider. Triggers credit release on delivery confirmation or credit reversal on RTO/cancellation.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| order_id | string | yes | - | External order ID |
| status | string | yes | - | Delivery status: "delivered", "rto", or "cancelled" |
| delivered_at | string | no | null | ISO 8601 timestamp of delivery |
| merchant_id | uuid | yes | - | Merchant ID |

### Example Request (Delivery Confirmed)

```bash
curl -X POST http://localhost:3000/cod/webhook/delivery \
  -H "Content-Type: application/json" \
  -d '{
    "order_id": "shopify_order_5003",
    "status": "delivered",
    "delivered_at": "2026-03-20T14:30:00Z",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
  }'
```

### Response

```json
{
  "status": "processed",
  "order_id": "shopify_order_5003"
}
```

### Example Request (RTO/Return)

```bash
curl -X POST http://localhost:3000/cod/webhook/delivery \
  -H "Content-Type: application/json" \
  -d '{
    "order_id": "shopify_order_5004",
    "status": "rto",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
  }'
```

### Response

```json
{
  "status": "processed",
  "order_id": "shopify_order_5004"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing merchant_id, missing order_id, or unknown status value |
| 404 | COD order not found |
| 500 | Database error |

---

## COD to Prepaid Incentive

**Method:** POST
**Path:** `/cod/incentive`

Awards an incentive when a customer converts a COD order to prepaid payment. This encourages customers to switch from COD to online payment methods.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| order_id | string | yes | - | Order being converted |
| customer_id | uuid | yes | - | Customer making the switch |
| order_amount | number | yes | - | Original order amount |
| new_payment_method | string | yes | - | New payment method (e.g. "razorpay", "upi") |

### Example Request

```bash
curl -X POST http://localhost:3000/cod/incentive \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "order_id": "shopify_order_5005",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "order_amount": 1999.0,
    "new_payment_method": "upi"
  }'
```

### Response

```json
{
  "incentive_amount": 50.0,
  "ledger_entry_id": "le56e7f8-a9b0-1234-c567-890123456789",
  "message": "Incentive credited for switching to prepaid"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 404 | Customer or wallet not found |
| 500 | Database error |

---

## List COD Orders

**Method:** GET
**Path:** `/cod/orders/{merchant_id}`

Returns COD orders for a merchant, optionally filtered by state.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| state | string | no | - | Filter by state: "pending", "delivered", "rto", "cancelled" |
| page | integer | no | 1 | Page number (minimum 1) |
| limit | integer | no | 50 | Items per page (1-100) |

### Example Request

```bash
curl "http://localhost:3000/cod/orders/a1b2c3d4-e5f6-7890-abcd-ef1234567890?state=pending&page=1&limit=20"
```

### Response

```json
[
  {
    "id": "co12a3b4-c5d6-7890-ef12-345678901234",
    "order_id": "shopify_order_5003",
    "state": "pending",
    "pending_amount": 0.0,
    "created_at": "2026-03-18T10:00:00Z"
  },
  {
    "id": "co23b4c5-d6e7-8901-f234-567890123456",
    "order_id": "shopify_order_5006",
    "state": "pending",
    "pending_amount": 0.0,
    "created_at": "2026-03-17T15:30:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |

---

## COD Analytics

**Method:** GET
**Path:** `/cod/analytics/{merchant_id}`

Returns aggregate COD metrics for a merchant.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |

### Example Request

```bash
curl http://localhost:3000/cod/analytics/a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Response

```json
{
  "total_pending": 45,
  "total_delivered": 312,
  "total_rto": 28,
  "pending_amount": 67500.0,
  "released_amount": 468000.0,
  "cancelled_amount": 42000.0
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |
