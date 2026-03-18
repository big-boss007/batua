# Wallets API

## Create Wallet

**Method:** POST
**Path:** `/wallets`

Creates a new wallet. A wallet belongs to a merchant and is optionally linked to a customer. Bearer wallets (not linked to a customer) use a bearer_code for identification.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant that owns the wallet |
| customer_id | uuid | no | null | Customer the wallet belongs to |
| is_bearer | boolean | yes | - | If true, wallet is not linked to a specific customer |
| bearer_code | string | no | null | Code for bearer wallets (required when is_bearer is true) |

### Example Request

```bash
curl -X POST http://localhost:3000/wallets \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "is_bearer": false
  }'
```

### Response

```json
{
  "wallet": {
    "id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "is_bearer": false,
    "bearer_code": null,
    "created_at": "2026-03-18T10:00:00Z"
  },
  "balance": null
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 409 | Wallet already exists for this merchant + customer combination |
| 500 | Database error |

---

## Get Wallet

**Method:** GET
**Path:** `/wallets/{id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Wallet ID |

### Example Request

```bash
curl http://localhost:3000/wallets/w1a2l3l4-e5t6-7890-abcd-ef1234567890
```

### Response

```json
{
  "wallet": {
    "id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "is_bearer": false,
    "bearer_code": null,
    "created_at": "2026-03-18T10:00:00Z"
  },
  "balance": null
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Wallet not found |
| 500 | Database error |

---

## Lookup Wallet

**Method:** GET
**Path:** `/wallets/lookup`

Looks up a wallet by merchant and customer combination.

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| customer_id | uuid | yes | - | Customer ID |

### Example Request

```bash
curl "http://localhost:3000/wallets/lookup?merchant_id=a1b2c3d4-e5f6-7890-abcd-ef1234567890&customer_id=c1d2e3f4-5678-90ab-cdef-123456789012"
```

### Response

```json
{
  "wallet": {
    "id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "is_bearer": false,
    "bearer_code": null,
    "created_at": "2026-03-18T10:00:00Z"
  },
  "balance": null
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing merchant_id or customer_id |
| 404 | Wallet not found for this merchant + customer |
| 500 | Database error |

---

## Get or Create Wallet

**Method:** POST
**Path:** `/wallets/get-or-create`

Returns the existing wallet for a merchant + customer pair, or creates one if it does not exist.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| customer_id | uuid | yes | - | Customer ID |

### Example Request

```bash
curl -X POST http://localhost:3000/wallets/get-or-create \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012"
  }'
```

### Response

```json
{
  "wallet": {
    "id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "is_bearer": false,
    "bearer_code": null,
    "created_at": "2026-03-18T10:00:00Z"
  },
  "balance": null
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 500 | Database error |

---

## List Wallets for Merchant

**Method:** GET
**Path:** `/merchants/{merchant_id}/wallets`

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
curl "http://localhost:3000/merchants/a1b2c3d4-e5f6-7890-abcd-ef1234567890/wallets?page=1&limit=10"
```

### Response

```json
[
  {
    "id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "is_bearer": false,
    "bearer_code": null,
    "created_at": "2026-03-18T10:00:00Z"
  },
  {
    "id": "w2b3c4d5-e6f7-8901-bcde-f23456789013",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "d2e3f4a5-6789-01bc-def0-234567890123",
    "is_bearer": false,
    "bearer_code": null,
    "created_at": "2026-03-17T15:30:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |
