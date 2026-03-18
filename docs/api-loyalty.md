# Loyalty API

Manages loyalty programs with tiered membership. Customers move between tiers based on configurable evaluation criteria (e.g. total spend, total orders) over a rolling period.

## Create Program

**Method:** POST
**Path:** `/loyalty/programs`

Creates a loyalty program for a merchant.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| name | string | yes | - | Program name (e.g. "Desi Threads Rewards Club") |
| evaluation_criteria | string | yes | - | What metric determines tier (e.g. "total_spend", "total_orders", "total_points") |
| evaluation_period_days | integer | no | null | Rolling window for evaluation (null = lifetime) |

### Example Request

```bash
curl -X POST http://localhost:3000/loyalty/programs \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "Desi Threads Rewards Club",
    "evaluation_criteria": "total_spend",
    "evaluation_period_days": 365
  }'
```

### Response

**Status:** 201 Created

```json
{
  "id": "lp12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "Desi Threads Rewards Club",
  "evaluation_criteria": "total_spend",
  "evaluation_period_days": 365,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 409 | Loyalty program already exists for this merchant |
| 500 | Database error |

---

## Get Program

**Method:** GET
**Path:** `/loyalty/programs/{merchant_id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |

### Example Request

```bash
curl http://localhost:3000/loyalty/programs/a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Response

```json
{
  "id": "lp12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "Desi Threads Rewards Club",
  "evaluation_criteria": "total_spend",
  "evaluation_period_days": 365,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | No loyalty program found for this merchant |
| 500 | Database error |

---

## Create Tier

**Method:** POST
**Path:** `/loyalty/tiers`

Adds a tier to a loyalty program.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| program_id | uuid | yes | - | Loyalty program ID |
| name | string | yes | - | Tier name (e.g. "Silver", "Gold", "Platinum") |
| rank | integer | yes | - | Tier rank (higher = better; used for ordering) |
| threshold | number | yes | - | Minimum qualifying value to reach this tier |
| earn_rate_multiplier | number | yes | - | Multiplier on earn rate for this tier (e.g. 1.5 for 1.5x) |
| benefits | object | yes | - | JSON object describing tier benefits |

### Example Request

```bash
curl -X POST http://localhost:3000/loyalty/tiers \
  -H "Content-Type: application/json" \
  -d '{
    "program_id": "lp12a3b4-c5d6-7890-ef12-345678901234",
    "name": "Gold",
    "rank": 2,
    "threshold": 10000.0,
    "earn_rate_multiplier": 1.5,
    "benefits": {
      "free_shipping": true,
      "early_access": true,
      "birthday_bonus": 500,
      "max_redemption_pct": 60
    }
  }'
```

### Response

**Status:** 201 Created

```json
{
  "id": "lt12a3b4-c5d6-7890-ef12-345678901234",
  "program_id": "lp12a3b4-c5d6-7890-ef12-345678901234",
  "name": "Gold",
  "rank": 2,
  "threshold": 10000.0,
  "earn_rate_multiplier": 1.5,
  "benefits": {
    "free_shipping": true,
    "early_access": true,
    "birthday_bonus": 500,
    "max_redemption_pct": 60
  },
  "created_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 404 | Program not found |
| 409 | Tier with same rank already exists in this program |
| 500 | Database error |

---

## Get Tiers

**Method:** GET
**Path:** `/loyalty/programs/{program_id}/tiers`

Returns all tiers for a loyalty program, ordered by rank.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| program_id | uuid | Loyalty program ID |

### Example Request

```bash
curl http://localhost:3000/loyalty/programs/lp12a3b4-c5d6-7890-ef12-345678901234/tiers
```

### Response

```json
[
  {
    "id": "lt01a2b3-c4d5-6789-ef01-234567890123",
    "program_id": "lp12a3b4-c5d6-7890-ef12-345678901234",
    "name": "Silver",
    "rank": 1,
    "threshold": 0.0,
    "earn_rate_multiplier": 1.0,
    "benefits": {
      "free_shipping": false,
      "early_access": false
    },
    "created_at": "2026-03-18T10:00:00Z"
  },
  {
    "id": "lt12a3b4-c5d6-7890-ef12-345678901234",
    "program_id": "lp12a3b4-c5d6-7890-ef12-345678901234",
    "name": "Gold",
    "rank": 2,
    "threshold": 10000.0,
    "earn_rate_multiplier": 1.5,
    "benefits": {
      "free_shipping": true,
      "early_access": true,
      "birthday_bonus": 500
    },
    "created_at": "2026-03-18T10:00:00Z"
  },
  {
    "id": "lt23b4c5-d6e7-8901-f234-567890123456",
    "program_id": "lp12a3b4-c5d6-7890-ef12-345678901234",
    "name": "Platinum",
    "rank": 3,
    "threshold": 50000.0,
    "earn_rate_multiplier": 2.0,
    "benefits": {
      "free_shipping": true,
      "early_access": true,
      "birthday_bonus": 1000,
      "max_redemption_pct": 75,
      "personal_stylist": true
    },
    "created_at": "2026-03-18T10:00:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |

---

## Get Customer Tier Info

**Method:** GET
**Path:** `/loyalty/customers/{merchant_id}/{customer_id}`

Returns a customer's current tier, program details, and progress toward the next tier.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |
| customer_id | uuid | Customer ID |

### Example Request

```bash
curl http://localhost:3000/loyalty/customers/a1b2c3d4-e5f6-7890-abcd-ef1234567890/c1d2e3f4-5678-90ab-cdef-123456789012
```

### Response

```json
{
  "customer": {
    "id": "ct12a3b4-c5d6-7890-ef12-345678901234",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "tier_id": "lt12a3b4-c5d6-7890-ef12-345678901234",
    "qualifying_value": 18500.0,
    "qualified_at": "2026-02-15T10:00:00Z",
    "expires_at": "2027-02-15T10:00:00Z",
    "created_at": "2026-01-10T08:00:00Z",
    "updated_at": "2026-02-15T10:00:00Z"
  },
  "tier": {
    "id": "lt12a3b4-c5d6-7890-ef12-345678901234",
    "program_id": "lp12a3b4-c5d6-7890-ef12-345678901234",
    "name": "Gold",
    "rank": 2,
    "threshold": 10000.0,
    "earn_rate_multiplier": 1.5,
    "benefits": {
      "free_shipping": true,
      "early_access": true,
      "birthday_bonus": 500
    },
    "created_at": "2026-03-18T10:00:00Z"
  },
  "program": {
    "id": "lp12a3b4-c5d6-7890-ef12-345678901234",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "Desi Threads Rewards Club",
    "evaluation_criteria": "total_spend",
    "evaluation_period_days": 365,
    "is_active": true,
    "created_at": "2026-03-18T10:00:00Z",
    "updated_at": "2026-03-18T10:00:00Z"
  },
  "progress_to_next": {
    "next_tier_name": "Platinum",
    "current_value": 18500.0,
    "threshold": 50000.0,
    "percentage": 37.0
  }
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | No tier found for this customer under this merchant |
| 500 | Database error |

---

## Evaluate Tier

**Method:** POST
**Path:** `/loyalty/evaluate/{merchant_id}/{customer_id}`

Re-evaluates a customer's tier based on current qualifying data. May result in a tier upgrade or downgrade.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |
| customer_id | uuid | Customer ID |

### Example Request

```bash
curl -X POST http://localhost:3000/loyalty/evaluate/a1b2c3d4-e5f6-7890-abcd-ef1234567890/c1d2e3f4-5678-90ab-cdef-123456789012
```

### Response (Tier Upgrade)

```json
{
  "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "current_tier": {
    "id": "lt12a3b4-c5d6-7890-ef12-345678901234",
    "program_id": "lp12a3b4-c5d6-7890-ef12-345678901234",
    "name": "Gold",
    "rank": 2,
    "threshold": 10000.0,
    "earn_rate_multiplier": 1.5,
    "benefits": {},
    "created_at": "2026-03-18T10:00:00Z"
  },
  "new_tier": {
    "id": "lt23b4c5-d6e7-8901-f234-567890123456",
    "program_id": "lp12a3b4-c5d6-7890-ef12-345678901234",
    "name": "Platinum",
    "rank": 3,
    "threshold": 50000.0,
    "earn_rate_multiplier": 2.0,
    "benefits": {},
    "created_at": "2026-03-18T10:00:00Z"
  },
  "changed": true,
  "direction": "upgrade"
}
```

### Response (No Change)

```json
{
  "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
  "current_tier": { "..." : "..." },
  "new_tier": null,
  "changed": false,
  "direction": null
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | No loyalty program or customer tier found |
| 500 | Database error |

---

## Get Tier Distribution

**Method:** GET
**Path:** `/loyalty/distribution/{merchant_id}`

Returns the distribution of customers across tiers for a merchant's loyalty program.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| merchant_id | uuid | Merchant ID |

### Example Request

```bash
curl http://localhost:3000/loyalty/distribution/a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Response

```json
[
  {
    "tier_name": "Silver",
    "count": 8500
  },
  {
    "tier_name": "Gold",
    "count": 1200
  },
  {
    "tier_name": "Platinum",
    "count": 150
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |
