# Rules & Campaigns API

Rules define how events translate into wallet credits. Campaigns apply time-bound multipliers or overrides on top of rules.

## Create Rule

**Method:** POST
**Path:** `/rules`

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| rule_type | string | yes | - | Rule type (e.g. "reward", "redemption", "notification") |
| name | string | yes | - | Human-readable rule name |
| config | object | yes | - | Rule configuration JSON |

The `config` object for a reward rule follows this structure:

| Field | Type | Description |
|-------|------|-------------|
| event_type | string | Event type this rule triggers on (e.g. "order.completed") |
| conditions | array | Array of condition objects |
| action | object | Reward action to execute when conditions match |

**Condition object:**

| Field | Type | Description |
|-------|------|-------------|
| field | string | Dot-path to the field in the event context |
| operator | string | Comparison operator (e.g. "gte", "eq", "in", "not_in") |
| value | any | Value to compare against |

**Action object:**

| Field | Type | Description |
|-------|------|-------------|
| bucket_type | string | Bucket to credit (e.g. "earned_credit") |
| calculation | string | Calculation method ("percentage", "fixed") |
| value | number | Reward value (percentage of order or fixed amount) |
| max_amount | number | Optional cap on the reward |
| conversion_rate | number | Optional override for earning-to-currency rate |
| expiry_days | integer | Optional days until credits expire |

### Example Request

```bash
curl -X POST http://localhost:3000/rules \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "rule_type": "reward",
    "name": "5% cashback on all orders",
    "config": {
      "event_type": "order.completed",
      "conditions": [
        {
          "field": "order_amount",
          "operator": "gte",
          "value": 500
        }
      ],
      "action": {
        "bucket_type": "earned_credit",
        "calculation": "percentage",
        "value": 5.0,
        "max_amount": 250.0,
        "conversion_rate": 1.0,
        "expiry_days": 90
      }
    }
  }'
```

### Response

```json
{
  "id": "r1u2l3e4-5678-90ab-cdef-123456789012",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "rule_type": "reward",
  "name": "5% cashback on all orders",
  "config": {
    "event_type": "order.completed",
    "conditions": [
      {
        "field": "order_amount",
        "operator": "gte",
        "value": 500
      }
    ],
    "action": {
      "bucket_type": "earned_credit",
      "calculation": "percentage",
      "value": 5.0,
      "max_amount": 250.0,
      "conversion_rate": 1.0,
      "expiry_days": 90
    }
  },
  "version": 1,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields or invalid config |
| 500 | Database error |

---

## Get Rule

**Method:** GET
**Path:** `/rules/{id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Rule ID |

### Example Request

```bash
curl http://localhost:3000/rules/r1u2l3e4-5678-90ab-cdef-123456789012
```

### Response

```json
{
  "id": "r1u2l3e4-5678-90ab-cdef-123456789012",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "rule_type": "reward",
  "name": "5% cashback on all orders",
  "config": {
    "event_type": "order.completed",
    "conditions": [],
    "action": {
      "bucket_type": "earned_credit",
      "calculation": "percentage",
      "value": 5.0,
      "max_amount": 250.0
    }
  },
  "version": 1,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Rule not found |
| 500 | Database error |

---

## List Rules

**Method:** GET
**Path:** `/rules`

Returns active rules for a merchant, optionally filtered by rule type.

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| rule_type | string | no | "reward" | Filter by rule type |

### Example Request

```bash
curl "http://localhost:3000/rules?merchant_id=a1b2c3d4-e5f6-7890-abcd-ef1234567890&rule_type=reward"
```

### Response

```json
[
  {
    "id": "r1u2l3e4-5678-90ab-cdef-123456789012",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "rule_type": "reward",
    "name": "5% cashback on all orders",
    "config": { "..." : "..." },
    "version": 1,
    "is_active": true,
    "created_at": "2026-03-18T10:00:00Z",
    "updated_at": "2026-03-18T10:00:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing merchant_id |
| 500 | Database error |

---

## Update Rule

**Method:** PUT
**Path:** `/rules/{id}`

Updates the config of a rule. This creates a new version and a new rule snapshot for auditability.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Rule ID |

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| config | object | yes | - | Updated rule configuration JSON |

### Example Request

```bash
curl -X PUT http://localhost:3000/rules/r1u2l3e4-5678-90ab-cdef-123456789012 \
  -H "Content-Type: application/json" \
  -d '{
    "config": {
      "event_type": "order.completed",
      "conditions": [
        {
          "field": "order_amount",
          "operator": "gte",
          "value": 300
        }
      ],
      "action": {
        "bucket_type": "earned_credit",
        "calculation": "percentage",
        "value": 7.0,
        "max_amount": 350.0,
        "conversion_rate": 1.0,
        "expiry_days": 60
      }
    }
  }'
```

### Response

```json
{
  "id": "r1u2l3e4-5678-90ab-cdef-123456789012",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "rule_type": "reward",
  "name": "5% cashback on all orders",
  "config": { "..." : "..." },
  "version": 2,
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:05:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Invalid config JSON |
| 404 | Rule not found |
| 500 | Database error |

---

## Evaluate Rules

**Method:** POST
**Path:** `/rules/evaluate`

Evaluates all active rules for a merchant against a given context. Returns matched rules with computed reward amounts.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| context | object | yes | - | Evaluation context |

**Context object:**

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| event_type | string | yes | - | Event type to evaluate |
| event_payload | object | yes | - | Full event payload |
| order_amount | number | no | null | Order total |
| payment_method | string | no | null | Payment method used |
| is_cod | boolean | yes | - | Whether order is cash-on-delivery |
| collections | string[] | yes | - | Product collections in the order |
| customer_tags | string[] | yes | - | Tags on the customer |
| is_first_order | boolean | yes | - | Whether this is the customer's first order |

### Example Request

```bash
curl -X POST http://localhost:3000/rules/evaluate \
  -H "Content-Type: application/json" \
  -d '{
    "context": {
      "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "event_type": "order.completed",
      "event_payload": {
        "total_price": "1499.00",
        "currency": "INR"
      },
      "order_amount": 1499.0,
      "payment_method": "razorpay",
      "is_cod": false,
      "collections": ["festive-collection", "sarees"],
      "customer_tags": ["vip"],
      "is_first_order": false
    }
  }'
```

### Response

```json
[
  {
    "matched": true,
    "rule_snapshot_id": "rs12a3b4-c5d6-7890-ef12-345678901234",
    "campaign_snapshot_id": null,
    "earning_unit": 74.95,
    "currency_equivalent": 74.95,
    "conversion_rate": 1.0,
    "bucket_type": "earned_credit",
    "expiry_days": 90,
    "constraints": {}
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing or invalid context |
| 500 | Database error |

---

## Create Campaign

**Method:** POST
**Path:** `/campaigns`

Creates a time-bound campaign that can apply multipliers on top of existing rules.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| name | string | yes | - | Campaign name |
| campaign_type | string | yes | - | Campaign type (e.g. "multiplier", "bonus", "festive") |
| config | object | yes | - | Campaign configuration JSON |
| base_rule_id | uuid | no | null | Rule to apply multiplier on |
| multiplier | number | no | null | Reward multiplier (e.g. 2.0 for 2x rewards) |
| starts_at | string | yes | - | ISO 8601 campaign start time |
| ends_at | string | yes | - | ISO 8601 campaign end time |

### Example Request

```bash
curl -X POST http://localhost:3000/campaigns \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "Diwali 2x Cashback",
    "campaign_type": "festive",
    "config": {
      "description": "Double rewards during Diwali week"
    },
    "base_rule_id": "r1u2l3e4-5678-90ab-cdef-123456789012",
    "multiplier": 2.0,
    "starts_at": "2026-10-20T00:00:00Z",
    "ends_at": "2026-10-27T23:59:59Z"
  }'
```

### Response

```json
{
  "id": "ca12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "Diwali 2x Cashback",
  "campaign_type": "festive",
  "config": {
    "description": "Double rewards during Diwali week"
  },
  "base_rule_id": "r1u2l3e4-5678-90ab-cdef-123456789012",
  "multiplier": 2.0,
  "starts_at": "2026-10-20T00:00:00Z",
  "ends_at": "2026-10-27T23:59:59Z",
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields or ends_at before starts_at |
| 500 | Database error |

---

## List Campaigns

**Method:** GET
**Path:** `/campaigns`

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| active_only | boolean | no | false | If true, only return currently active campaigns |

### Example Request

```bash
curl "http://localhost:3000/campaigns?merchant_id=a1b2c3d4-e5f6-7890-abcd-ef1234567890&active_only=true"
```

### Response

```json
[
  {
    "id": "ca12a3b4-c5d6-7890-ef12-345678901234",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "Diwali 2x Cashback",
    "campaign_type": "festive",
    "config": {},
    "base_rule_id": "r1u2l3e4-5678-90ab-cdef-123456789012",
    "multiplier": 2.0,
    "starts_at": "2026-10-20T00:00:00Z",
    "ends_at": "2026-10-27T23:59:59Z",
    "is_active": true,
    "created_at": "2026-03-18T10:00:00Z",
    "updated_at": "2026-03-18T10:00:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing merchant_id |
| 500 | Database error |

---

## List Campaign Templates

**Method:** GET
**Path:** `/campaigns/templates`

Returns a list of built-in festive and seasonal campaign templates that merchants can use as starting points.

### Example Request

```bash
curl http://localhost:3000/campaigns/templates
```

### Response

```json
[
  {
    "name": "diwali",
    "display_name": "Diwali Dhamaka",
    "description": "Double rewards during Diwali week",
    "default_multiplier": 2.0,
    "default_duration_days": 7,
    "suggested_start": "2026-10-20",
    "category": "festive"
  },
  {
    "name": "holi",
    "display_name": "Holi Colors Sale",
    "description": "Bonus rewards for Holi celebrations",
    "default_multiplier": 1.5,
    "default_duration_days": 3,
    "suggested_start": "2027-03-14",
    "category": "festive"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| (none) | This endpoint always returns 200 |

---

## Create Campaign from Template

**Method:** POST
**Path:** `/campaigns/from-template`

Creates a campaign based on a predefined template.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| template_name | string | yes | - | Template name (from list templates) |
| base_rule_id | uuid | yes | - | Rule to apply the campaign multiplier on |
| multiplier | number | no | template default | Override the template's default multiplier |
| starts_at | string | yes | - | ISO 8601 campaign start time |
| ends_at | string | yes | - | ISO 8601 campaign end time |
| custom_name | string | no | null | Custom name override |

### Example Request

```bash
curl -X POST http://localhost:3000/campaigns/from-template \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "template_name": "diwali",
    "base_rule_id": "r1u2l3e4-5678-90ab-cdef-123456789012",
    "multiplier": 3.0,
    "starts_at": "2026-10-20T00:00:00Z",
    "ends_at": "2026-10-27T23:59:59Z",
    "custom_name": "Desi Threads Diwali 3x"
  }'
```

### Response

```json
{
  "id": "ca34b5c6-d7e8-9012-f345-678901234567",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "Desi Threads Diwali 3x",
  "campaign_type": "festive",
  "config": {
    "template": "diwali",
    "description": "Double rewards during Diwali week"
  },
  "base_rule_id": "r1u2l3e4-5678-90ab-cdef-123456789012",
  "multiplier": 3.0,
  "starts_at": "2026-10-20T00:00:00Z",
  "ends_at": "2026-10-27T23:59:59Z",
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Unknown template_name or missing required fields |
| 500 | Database error |

---

## Campaign Calendar

**Method:** GET
**Path:** `/campaigns/calendar`

Returns campaigns in a calendar view with running status.

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| from | string | no | - | ISO 8601 start date filter |
| to | string | no | - | ISO 8601 end date filter |

### Example Request

```bash
curl "http://localhost:3000/campaigns/calendar?merchant_id=a1b2c3d4-e5f6-7890-abcd-ef1234567890&from=2026-10-01&to=2026-12-31"
```

### Response

```json
[
  {
    "id": "ca12a3b4-c5d6-7890-ef12-345678901234",
    "name": "Diwali 2x Cashback",
    "campaign_type": "festive",
    "multiplier": 2.0,
    "starts_at": "2026-10-20T00:00:00Z",
    "ends_at": "2026-10-27T23:59:59Z",
    "is_active": true,
    "is_currently_running": false
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing merchant_id |
| 500 | Database error |

---

## Campaign Performance

**Method:** GET
**Path:** `/campaigns/{id}/performance`

Returns performance metrics for a specific campaign.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Campaign ID |

### Example Request

```bash
curl http://localhost:3000/campaigns/ca12a3b4-c5d6-7890-ef12-345678901234/performance
```

### Response

```json
{
  "campaign_id": "ca12a3b4-c5d6-7890-ef12-345678901234",
  "name": "Diwali 2x Cashback",
  "total_entries": 342,
  "total_value": 28500.0,
  "unique_customers": 215,
  "average_reward": 83.33
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Campaign not found |
| 500 | Database error |
