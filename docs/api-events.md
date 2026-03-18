# Events API

## Ingest Event

**Method:** POST
**Path:** `/events/ingest`

Ingests an event into the system. Events are the triggers for reward rules evaluation. Duplicate events (same merchant + external_event_id) are detected and returned without reprocessing.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant that owns the event |
| event_type | string | yes | - | Type of event (e.g. "order.completed", "order.refunded", "signup") |
| event_source | string | yes | - | Source system (e.g. "shopify", "api", "manual") |
| external_event_id | string | yes | - | Unique ID from the source system |
| payload | object | yes | - | Full event payload as JSON |

### Example Request

```bash
curl -X POST http://localhost:3000/events/ingest \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "event_type": "order.completed",
    "event_source": "shopify",
    "external_event_id": "shopify_order_5001",
    "payload": {
      "order_id": "5001",
      "total_price": "1499.00",
      "currency": "INR",
      "customer": {
        "phone": "+919876543210",
        "email": "priya.sharma@gmail.com"
      },
      "line_items": [
        {
          "title": "Block Print Kurta",
          "quantity": 1,
          "price": "1499.00"
        }
      ]
    }
  }'
```

### Response

```json
{
  "event_id": "ev12a3b4-c5d6-7890-ef12-345678901234",
  "state": "Received",
  "is_duplicate": false
}
```

### Response (Duplicate)

```json
{
  "event_id": "ev12a3b4-c5d6-7890-ef12-345678901234",
  "state": "Processed",
  "is_duplicate": true
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 500 | Database error |

---

## Shopify Order Webhook

**Method:** POST
**Path:** `/events/shopify/orders`

Receives a Shopify order webhook payload and converts it into a standard event.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID for the Shopify store |
| payload | object | yes | - | Raw Shopify order webhook JSON |

The payload is expected to contain standard Shopify order fields: `id`, `order_number`, `email`, `phone`, `total_price`, `currency`, `financial_status`, `gateway`, `payment_gateway_names`, `customer`, and `line_items`.

### Example Request

```bash
curl -X POST http://localhost:3000/events/shopify/orders \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "payload": {
      "id": 5001234567890,
      "order_number": 1042,
      "email": "priya.sharma@gmail.com",
      "phone": "+919876543210",
      "total_price": "2999.00",
      "currency": "INR",
      "financial_status": "paid",
      "gateway": "razorpay",
      "payment_gateway_names": ["razorpay"],
      "customer": {
        "id": 7001234567890,
        "email": "priya.sharma@gmail.com",
        "phone": "+919876543210",
        "first_name": "Priya",
        "last_name": "Sharma"
      },
      "line_items": [
        {
          "title": "Banarasi Silk Saree",
          "quantity": 1,
          "price": "2999.00"
        }
      ]
    }
  }'
```

### Response

```json
{
  "event_id": "ev23b4c5-d6e7-8901-f234-567890123456",
  "state": "Received",
  "is_duplicate": false
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing merchant_id or invalid Shopify payload |
| 500 | Database error |

---

## Get Event

**Method:** GET
**Path:** `/events/{id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Event ID |

### Example Request

```bash
curl http://localhost:3000/events/ev12a3b4-c5d6-7890-ef12-345678901234
```

### Response

```json
{
  "id": "ev12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "event_type": "order.completed",
  "event_source": "shopify",
  "external_event_id": "shopify_order_5001",
  "payload": {
    "order_id": "5001",
    "total_price": "1499.00",
    "currency": "INR"
  },
  "state": "Processed",
  "idempotency_key": "a1b2c3d4:shopify_order_5001",
  "created_at": "2026-03-18T10:00:00Z",
  "processed_at": "2026-03-18T10:00:01Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Event not found |
| 500 | Database error |

---

## List Events

**Method:** GET
**Path:** `/events`

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | no | - | Filter by merchant |
| event_type | string | no | - | Filter by event type (e.g. "order.completed") |
| state | string | no | - | Filter by state: "received", "processing", "processed", "failed", "duplicate" |
| limit | integer | no | 50 | Max results (capped at 100) |
| offset | integer | no | 0 | Number of results to skip |

### Example Request

```bash
curl "http://localhost:3000/events?merchant_id=a1b2c3d4-e5f6-7890-abcd-ef1234567890&event_type=order.completed&limit=10"
```

### Response

```json
[
  {
    "id": "ev12a3b4-c5d6-7890-ef12-345678901234",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "event_type": "order.completed",
    "event_source": "shopify",
    "external_event_id": "shopify_order_5001",
    "payload": {
      "order_id": "5001",
      "total_price": "1499.00"
    },
    "state": "Processed",
    "idempotency_key": "a1b2c3d4:shopify_order_5001",
    "created_at": "2026-03-18T10:00:00Z",
    "processed_at": "2026-03-18T10:00:01Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |
