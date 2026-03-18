# Identity API

## Resolve Identity

**Method:** POST
**Path:** `/identity/resolve`

Resolves a customer by phone number. If the customer does not exist, creates a new one.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| phone | string | yes | - | Customer phone number (primary identifier) |
| email | string | no | null | Customer email address |
| name | string | no | null | Customer full name |
| external_id | string | no | null | ID from the external system (e.g. Shopify customer ID) |

### Example Request

```bash
curl -X POST http://localhost:3000/identity/resolve \
  -H "Content-Type: application/json" \
  -d '{
    "phone": "+919876543210",
    "email": "priya.sharma@gmail.com",
    "name": "Priya Sharma",
    "external_id": "shopify_cust_98765"
  }'
```

### Response (Existing Customer)

**Status:** 200 OK

```json
{
  "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "is_verified": true,
  "is_new": false
}
```

### Response (New Customer)

**Status:** 201 Created

```json
{
  "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "is_verified": false,
  "is_new": true
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing phone number |
| 500 | Database error |

---

## Get Customer

**Method:** GET
**Path:** `/identity/customers/{id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Customer ID |

### Example Request

```bash
curl http://localhost:3000/identity/customers/c1d2e3f4-5678-90ab-cdef-123456789012
```

### Response

```json
{
  "id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "phone": "+919876543210",
  "email": "priya.sharma@gmail.com",
  "name": "Priya Sharma",
  "external_id": "shopify_cust_98765",
  "is_verified": true,
  "created_at": "2026-03-01T08:30:00Z",
  "updated_at": "2026-03-15T14:20:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Customer not found |
| 500 | Database error |

---

## Update Customer

**Method:** PUT
**Path:** `/identity/customers/{id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Customer ID |

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| email | string | no | - | Updated email address |
| name | string | no | - | Updated name |
| external_id | string | no | - | Updated external system ID |
| is_verified | boolean | no | - | Verification status |

### Example Request

```bash
curl -X PUT http://localhost:3000/identity/customers/c1d2e3f4-5678-90ab-cdef-123456789012 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Priya Sharma",
    "is_verified": true
  }'
```

### Response

```json
{
  "id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "phone": "+919876543210",
  "email": "priya.sharma@gmail.com",
  "name": "Priya Sharma",
  "external_id": "shopify_cust_98765",
  "is_verified": true,
  "created_at": "2026-03-01T08:30:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Customer not found |
| 500 | Database error |

---

## Search Customers

**Method:** GET
**Path:** `/identity/customers`

Search for customers by phone number or external ID.

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| phone | string | no | - | Phone number to search |
| external_id | string | no | - | External ID to search |

At least one of `phone` or `external_id` should be provided.

### Example Request

```bash
curl "http://localhost:3000/identity/customers?phone=%2B919876543210"
```

### Response

```json
[
  {
    "id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "phone": "+919876543210",
    "email": "priya.sharma@gmail.com",
    "name": "Priya Sharma",
    "external_id": "shopify_cust_98765",
    "is_verified": true,
    "created_at": "2026-03-01T08:30:00Z",
    "updated_at": "2026-03-15T14:20:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |
