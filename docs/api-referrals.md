# Referrals API

Manages referral programs with fraud detection. Supports vanity codes, creator commissions, and automatic reward distribution to both referrer and referee.

## Create Program

**Method:** POST
**Path:** `/referrals/programs`

Creates a referral program for a merchant.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| referrer_reward_amount | number | yes | - | Amount credited to the referrer on successful conversion |
| referee_reward_amount | number | yes | - | Amount credited to the referee (new customer) |
| max_referrals_per_customer | integer | no | null | Cap on referrals per customer (null = unlimited) |

### Example Request

```bash
curl -X POST http://localhost:3000/referrals/programs \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "referrer_reward_amount": 200.0,
    "referee_reward_amount": 100.0,
    "max_referrals_per_customer": 50
  }'
```

### Response

**Status:** 201 Created

```json
{
  "id": "rp12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "referrer_reward_amount": 200.0,
  "referee_reward_amount": 100.0,
  "referrer_bucket_type": "ReferralReward",
  "referee_bucket_type": "ReferralReward",
  "max_referrals_per_customer": 50,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 409 | Referral program already exists for this merchant |
| 500 | Database error |

---

## Get Program

**Method:** GET
**Path:** `/referrals/programs/{merchant_id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |

### Example Request

```bash
curl http://localhost:3000/referrals/programs/a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Response

```json
{
  "id": "rp12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "referrer_reward_amount": 200.0,
  "referee_reward_amount": 100.0,
  "referrer_bucket_type": "ReferralReward",
  "referee_bucket_type": "ReferralReward",
  "max_referrals_per_customer": 50,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | No referral program found for this merchant |
| 500 | Database error |

---

## Create Referral Code

**Method:** POST
**Path:** `/referrals/codes`

Creates a referral code for a customer. Supports vanity codes (auto-generated from customer name) and creator codes (with commission rate).

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| customer_id | uuid | yes | - | Customer the code belongs to |
| code | string | no | null | Custom code (auto-generated if not provided) |
| is_vanity | boolean | yes | - | Whether to generate a vanity code from customer name |
| is_creator | boolean | yes | - | Whether this is a creator/influencer code |
| commission_rate | number | no | null | Commission rate for creators (e.g. 0.05 for 5%) |

### Example Request (Vanity Code)

```bash
curl -X POST http://localhost:3000/referrals/codes \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "is_vanity": true,
    "is_creator": false
  }'
```

### Response

**Status:** 201 Created

```json
{
  "id": "rc12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "code": "PRIYA42",
  "is_vanity": true,
  "is_creator": false,
  "commission_rate": null,
  "total_referrals": 0,
  "total_conversions": 0,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z"
}
```

### Example Request (Creator Code)

```bash
curl -X POST http://localhost:3000/referrals/codes \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "d2e3f4a5-6789-01bc-def0-234567890123",
    "code": "FASHIONBYNEHA",
    "is_vanity": false,
    "is_creator": true,
    "commission_rate": 0.05
  }'
```

### Response

**Status:** 201 Created

```json
{
  "id": "rc23b4c5-d6e7-8901-f234-567890123456",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "customer_id": "d2e3f4a5-6789-01bc-def0-234567890123",
  "code": "FASHIONBYNEHA",
  "is_vanity": false,
  "is_creator": true,
  "commission_rate": 0.05,
  "total_referrals": 0,
  "total_conversions": 0,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 404 | Customer not found (for vanity code generation) |
| 409 | Code already exists |
| 500 | Database error |

---

## Get Referral Code

**Method:** GET
**Path:** `/referrals/codes/{code}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| code | string | Referral code |

### Example Request

```bash
curl http://localhost:3000/referrals/codes/PRIYA42
```

### Response

```json
{
  "id": "rc12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "code": "PRIYA42",
  "is_vanity": true,
  "is_creator": false,
  "commission_rate": null,
  "total_referrals": 5,
  "total_conversions": 3,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Referral code not found |
| 500 | Database error |

---

## Convert Referral

**Method:** POST
**Path:** `/referrals/convert`

Processes a referral conversion. Validates the referral, runs fraud checks (IP, device fingerprint, self-referral), and credits both the referrer and referee if the conversion is legitimate.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| referral_code | string | yes | - | The referral code used |
| referee_id | uuid | yes | - | Customer ID of the person who used the code |
| order_id | string | no | null | Order ID that triggered the conversion |
| referee_ip | string | no | null | IP address of the referee (for fraud detection) |
| referee_device_fingerprint | string | no | null | Device fingerprint (for fraud detection) |

### Example Request

```bash
curl -X POST http://localhost:3000/referrals/convert \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "referral_code": "PRIYA42",
    "referee_id": "e3f4a5b6-7890-12cd-ef01-345678901234",
    "order_id": "shopify_order_5015",
    "referee_ip": "103.21.58.192",
    "referee_device_fingerprint": "fp_abc123def456"
  }'
```

### Response

```json
{
  "conversion_id": "rv12a3b4-c5d6-7890-ef12-345678901234",
  "referrer_rewarded": true,
  "referee_rewarded": true,
  "fraud_signals": []
}
```

### Response (Suspicious Conversion)

When fraud signals are detected, rewards may be withheld:

```json
{
  "conversion_id": "rv23b4c5-d6e7-8901-f234-567890123456",
  "referrer_rewarded": false,
  "referee_rewarded": false,
  "fraud_signals": [
    "same_ip_as_referrer",
    "same_device_as_referrer"
  ]
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields or self-referral attempt |
| 404 | Referral code not found or inactive |
| 409 | Referee already converted with this code |
| 500 | Database error |

---

## Get Referral Analytics

**Method:** GET
**Path:** `/referrals/analytics/{merchant_id}`

Returns aggregate referral metrics for a merchant.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |

### Example Request

```bash
curl http://localhost:3000/referrals/analytics/a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Response

```json
{
  "total_codes": 342,
  "total_referrals": 1250,
  "total_conversions": 890,
  "total_suspicious": 12,
  "conversion_rate": 71.2
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |

---

## List Conversions

**Method:** GET
**Path:** `/referrals/conversions/{merchant_id}`

Returns individual referral conversions for a merchant.

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
curl "http://localhost:3000/referrals/conversions/a1b2c3d4-e5f6-7890-abcd-ef1234567890?page=1&limit=10"
```

### Response

```json
[
  {
    "id": "rv12a3b4-c5d6-7890-ef12-345678901234",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "referral_code_id": "rc12a3b4-c5d6-7890-ef12-345678901234",
    "referrer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "referee_id": "e3f4a5b6-7890-12cd-ef01-345678901234",
    "order_id": "shopify_order_5015",
    "referrer_entry_id": "le78a9b0-c1d2-3456-e789-012345678901",
    "referee_entry_id": "le89b0c1-d2e3-4567-f890-123456789012",
    "referee_ip": "103.21.58.192",
    "referee_device_fingerprint": "fp_abc123def456",
    "is_suspicious": false,
    "fraud_signals": {},
    "created_at": "2026-03-18T10:00:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |
