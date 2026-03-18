# Admin API

## Create Merchant

**Method:** POST
**Path:** `/admin/merchants`

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| external_id | string | yes | - | Unique identifier from the external system (e.g. Shopify shop ID) |
| name | string | yes | - | Merchant display name |
| domain | string | no | null | Merchant's primary domain |
| currency | string | no | "INR" | ISO 4217 currency code |
| timezone | string | no | "Asia/Kolkata" | IANA timezone |

### Example Request

```bash
curl -X POST http://localhost:3000/admin/merchants \
  -H "Content-Type: application/json" \
  -d '{
    "external_id": "shopify_12345",
    "name": "Desi Threads",
    "domain": "desithreads.in",
    "currency": "INR",
    "timezone": "Asia/Kolkata"
  }'
```

### Response

```json
{
  "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "external_id": "shopify_12345",
  "name": "Desi Threads",
  "domain": "desithreads.in",
  "currency": "INR",
  "timezone": "Asia/Kolkata",
  "is_active": true,
  "geo_policy_id": null,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields (external_id, name) |
| 409 | Merchant with the same external_id already exists |
| 500 | Database error |

---

## Get Merchant

**Method:** GET
**Path:** `/admin/merchants/{id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Merchant ID |

### Example Request

```bash
curl http://localhost:3000/admin/merchants/a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Response

```json
{
  "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "external_id": "shopify_12345",
  "name": "Desi Threads",
  "domain": "desithreads.in",
  "currency": "INR",
  "timezone": "Asia/Kolkata",
  "is_active": true,
  "geo_policy_id": null,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Merchant not found |
| 500 | Database error |

---

## List Merchants

**Method:** GET
**Path:** `/admin/merchants`

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| page | integer | no | 1 | Page number (minimum 1) |
| limit | integer | no | 20 | Items per page (1-100) |

### Example Request

```bash
curl "http://localhost:3000/admin/merchants?page=1&limit=10"
```

### Response

```json
[
  {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "external_id": "shopify_12345",
    "name": "Desi Threads",
    "domain": "desithreads.in",
    "currency": "INR",
    "timezone": "Asia/Kolkata",
    "is_active": true,
    "geo_policy_id": null,
    "created_at": "2026-03-18T10:00:00Z",
    "updated_at": "2026-03-18T10:00:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |

---

## Update Merchant

**Method:** PUT
**Path:** `/admin/merchants/{id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Merchant ID |

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| name | string | no | - | Updated display name |
| domain | string | no | - | Updated domain |
| is_active | boolean | no | - | Enable or disable the merchant |

### Example Request

```bash
curl -X PUT http://localhost:3000/admin/merchants/a1b2c3d4-e5f6-7890-abcd-ef1234567890 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Desi Threads Official",
    "is_active": true
  }'
```

### Response

```json
{
  "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "external_id": "shopify_12345",
  "name": "Desi Threads Official",
  "domain": "desithreads.in",
  "currency": "INR",
  "timezone": "Asia/Kolkata",
  "is_active": true,
  "geo_policy_id": null,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:05:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Merchant not found |
| 500 | Database error |

---

## Bulk Credit

**Method:** POST
**Path:** `/admin/bulk-credit`

Credits wallet balances for multiple customers in a single operation.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant owning the wallets |
| customer_ids | uuid[] | yes | - | List of customer IDs to credit |
| amount | number | yes | - | Credit amount per customer (currency equivalent) |
| bucket_type | string | yes | - | Bucket to credit (e.g. "earned_credit", "goodwill_credit") |
| reason | string | yes | - | Audit reason for the credit |
| actor_id | string | yes | - | ID of the admin performing the action |

### Example Request

```bash
curl -X POST http://localhost:3000/admin/bulk-credit \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_ids": [
      "c1d2e3f4-5678-90ab-cdef-123456789012",
      "d2e3f4a5-6789-01bc-def0-234567890123"
    ],
    "amount": 100.0,
    "bucket_type": "goodwill_credit",
    "reason": "Diwali promotion bonus",
    "actor_id": "admin_ravi"
  }'
```

### Response

```json
{
  "total_processed": 2,
  "total_succeeded": 2,
  "total_failed": 0,
  "results": [
    {
      "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
      "success": true,
      "ledger_entry_id": "e1f2a3b4-c5d6-7890-ef12-345678901234",
      "error": null
    },
    {
      "customer_id": "d2e3f4a5-6789-01bc-def0-234567890123",
      "success": true,
      "ledger_entry_id": "f2a3b4c5-d6e7-8901-f234-567890123456",
      "error": null
    }
  ]
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields or empty customer_ids |
| 404 | Wallet not found for one or more customers (partial success possible) |
| 500 | Database error |

---

## Process Dispute

**Method:** POST
**Path:** `/admin/disputes`

Reverses a ledger entry due to a dispute. Creates a compensating entry.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| customer_id | uuid | yes | - | Customer who raised the dispute |
| ledger_entry_id | uuid | yes | - | The ledger entry to reverse |
| reason | string | yes | - | Reason for the dispute |
| actor_id | string | yes | - | ID of the admin processing the dispute |

### Example Request

```bash
curl -X POST http://localhost:3000/admin/disputes \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "ledger_entry_id": "e1f2a3b4-c5d6-7890-ef12-345678901234",
    "reason": "Order was never delivered",
    "actor_id": "admin_ravi"
  }'
```

### Response

```json
{
  "reversal_entry_id": "a9b8c7d6-e5f4-3210-abcd-ef9876543210",
  "original_amount": 50.0,
  "reversed": true
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 404 | Ledger entry not found |
| 409 | Entry already reversed |
| 500 | Database error |

---

## Create Wallet Policy

**Method:** POST
**Path:** `/admin/wallet-policies`

Configures redemption and earning policies for a specific bucket type.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| bucket_type | string | yes | - | Bucket type (e.g. "earned_credit", "gift_card") |
| min_redemption | number | no | null | Minimum amount for redemption |
| step_size | number | no | null | Redemption must be in multiples of this value |
| max_per_order_pct | number | no | null | Maximum percentage of order value redeemable |
| max_per_order_fixed | number | no | null | Maximum fixed amount redeemable per order |
| stackable_with_discounts | boolean | no | null | Whether credits can stack with discount codes |
| default_expiry_days | integer | no | null | Default expiry in days for credits in this bucket |
| is_transferable | boolean | no | null | Whether credits in this bucket can be transferred |

### Example Request

```bash
curl -X POST http://localhost:3000/admin/wallet-policies \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "bucket_type": "earned_credit",
    "min_redemption": 10.0,
    "step_size": 1.0,
    "max_per_order_pct": 50.0,
    "max_per_order_fixed": 500.0,
    "stackable_with_discounts": false,
    "default_expiry_days": 90,
    "is_transferable": false
  }'
```

### Response

```json
{
  "status": "ok"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields or invalid bucket_type |
| 409 | Policy already exists for this merchant + bucket_type |
| 500 | Database error |

---

## List Wallet Policies

**Method:** GET
**Path:** `/admin/wallet-policies/{merchant_id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |

### Example Request

```bash
curl http://localhost:3000/admin/wallet-policies/a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Response

```json
[
  {
    "id": "p1o2l3i4-c5y6-7890-abcd-ef1234567890",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "bucket_type": "EarnedCredit",
    "min_redemption": 10.0,
    "step_size": 1.0,
    "max_per_order_pct": 50.0,
    "max_per_order_fixed": 500.0,
    "stackable_with_discounts": false,
    "default_conversion_rate": 1.0,
    "default_expiry_days": 90,
    "is_transferable": false,
    "excluded_payment_methods": [],
    "excluded_collections": [],
    "is_active": true,
    "created_at": "2026-03-18T10:00:00Z",
    "updated_at": "2026-03-18T10:00:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |

---

## Create Geo Policy

**Method:** POST
**Path:** `/admin/geo-policies`

Creates a geographic policy governing wallet behavior in a specific region.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| geo_code | string | yes | - | ISO 3166 country/region code (e.g. "IN", "US") |
| name | string | yes | - | Human-readable policy name |
| config | object | yes | - | JSON configuration for the policy |

### Example Request

```bash
curl -X POST http://localhost:3000/admin/geo-policies \
  -H "Content-Type: application/json" \
  -d '{
    "geo_code": "IN",
    "name": "India Default Policy",
    "config": {
      "max_wallet_balance": 10000,
      "kyc_required_above": 2000,
      "ppi_compliance": true
    }
  }'
```

### Response

```json
{
  "id": "g1e2o3p4-o5l6-7890-abcd-ef1234567890",
  "geo_code": "IN",
  "name": "India Default Policy",
  "config": {
    "max_wallet_balance": 10000,
    "kyc_required_above": 2000,
    "ppi_compliance": true
  },
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 409 | Geo policy for this geo_code already exists |
| 500 | Database error |

---

## Get Geo Policy

**Method:** GET
**Path:** `/admin/geo-policies/{geo_code}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| geo_code | string | ISO 3166 country/region code |

### Example Request

```bash
curl http://localhost:3000/admin/geo-policies/IN
```

### Response

```json
{
  "id": "g1e2o3p4-o5l6-7890-abcd-ef1234567890",
  "geo_code": "IN",
  "name": "India Default Policy",
  "config": {
    "max_wallet_balance": 10000,
    "kyc_required_above": 2000,
    "ppi_compliance": true
  },
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Geo policy not found for the given geo_code |
| 500 | Database error |

---

## Dashboard

**Method:** GET
**Path:** `/admin/dashboard`

Returns system-wide statistics for the admin dashboard.

### Example Request

```bash
curl http://localhost:3000/admin/dashboard
```

### Response

```json
{
  "total_merchants": 42,
  "total_wallets": 15230,
  "total_ledger_entries": 89401,
  "total_value_in_system": 4523100.50
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |
