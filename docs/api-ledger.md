# Ledger API

## Create Entry

**Method:** POST
**Path:** `/entries`

Creates a new ledger entry. This is the core primitive for all wallet balance changes. Each entry records a movement of value into, out of, or across a wallet.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| wallet_id | uuid | yes | - | Wallet to create the entry for |
| bucket_type | string | yes | - | One of: "earned_credit", "cod_pending", "gift_card", "customer_funded", "referral_reward", "goodwill_credit", "membership_benefit", "refund_credit" |
| movement_type | string | yes | - | One of: "in", "held", "out", "across" |
| earning_unit | number | yes | - | Amount in earning units (e.g. points, coins) |
| currency_equivalent | number | yes | - | Amount in currency (e.g. INR) |
| conversion_rate | number | yes | - | Rate to convert earning_unit to currency_equivalent |
| event_id | uuid | no | null | Associated event that triggered this entry |
| rule_snapshot_id | uuid | no | null | Rule snapshot used to compute the reward |
| campaign_snapshot_id | uuid | no | null | Campaign snapshot applied |
| actor_type | string | yes | - | One of: "system", "human", "automation", "migration" |
| actor_id | string | no | null | Identifier of the actor (e.g. admin username) |
| payment_reference | string | no | null | External payment reference (e.g. Shopify order ID) |
| transfer_id | uuid | no | null | Transfer ID for cross-wallet movements |
| constraints | object | no | {} | JSON constraints on how this credit can be used |
| expires_at | string | no | null | ISO 8601 expiration timestamp |
| idempotency_key | string | no | auto-generated | Client-supplied idempotency key to prevent duplicates |

### Example Request

```bash
curl -X POST http://localhost:3000/entries \
  -H "Content-Type: application/json" \
  -d '{
    "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
    "bucket_type": "earned_credit",
    "movement_type": "in",
    "earning_unit": 50.0,
    "currency_equivalent": 50.0,
    "conversion_rate": 1.0,
    "event_id": "ev12a3b4-c5d6-7890-ef12-345678901234",
    "rule_snapshot_id": "rs12a3b4-c5d6-7890-ef12-345678901234",
    "actor_type": "system",
    "expires_at": "2026-06-18T10:00:00Z"
  }'
```

### Response

**Status:** 201 Created

```json
{
  "id": "le12a3b4-c5d6-7890-ef12-345678901234",
  "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
  "bucket_type": "EarnedCredit",
  "movement_type": "In",
  "earning_unit": 50.0,
  "currency_equivalent": 50.0,
  "conversion_rate": 1.0,
  "idempotency_key": "w1a2l3l4-e5t6-7890-abcd-ef1234567890:ev12a3b4:rs12a3b4",
  "event_id": "ev12a3b4-c5d6-7890-ef12-345678901234",
  "rule_snapshot_id": "rs12a3b4-c5d6-7890-ef12-345678901234",
  "campaign_snapshot_id": null,
  "actor_type": "System",
  "actor_id": null,
  "payment_reference": null,
  "transfer_id": null,
  "constraints": {},
  "expires_at": "2026-06-18T10:00:00Z",
  "created_at": "2026-03-18T10:00:00Z",
  "state": "Active"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields or invalid enum values |
| 409 | Duplicate idempotency_key (entry already exists) |
| 404 | Wallet not found |
| 500 | Database error |

---

## List Entries

**Method:** GET
**Path:** `/wallets/{wallet_id}/entries`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| wallet_id | uuid | Wallet ID |

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| page | integer | no | 1 | Page number |
| limit | integer | no | 50 | Items per page |
| bucket_type | string | no | - | Filter by bucket type |
| movement_type | string | no | - | Filter by movement type ("in", "held", "out", "across") |

### Example Request

```bash
curl "http://localhost:3000/wallets/w1a2l3l4-e5t6-7890-abcd-ef1234567890/entries?page=1&limit=10&bucket_type=earned_credit"
```

### Response

```json
[
  {
    "id": "le12a3b4-c5d6-7890-ef12-345678901234",
    "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
    "bucket_type": "EarnedCredit",
    "movement_type": "In",
    "earning_unit": 50.0,
    "currency_equivalent": 50.0,
    "conversion_rate": 1.0,
    "idempotency_key": "w1a2l3l4:ev12a3b4:rs12a3b4",
    "event_id": "ev12a3b4-c5d6-7890-ef12-345678901234",
    "rule_snapshot_id": "rs12a3b4-c5d6-7890-ef12-345678901234",
    "campaign_snapshot_id": null,
    "actor_type": "System",
    "actor_id": null,
    "payment_reference": null,
    "transfer_id": null,
    "constraints": {},
    "expires_at": "2026-06-18T10:00:00Z",
    "created_at": "2026-03-18T10:00:00Z",
    "state": "Active"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |

---

## Get Balance

**Method:** GET
**Path:** `/wallets/{wallet_id}/balance`

Returns the current balance of a wallet, broken down by bucket type.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| wallet_id | uuid | Wallet ID |

### Example Request

```bash
curl http://localhost:3000/wallets/w1a2l3l4-e5t6-7890-abcd-ef1234567890/balance
```

### Response

```json
{
  "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
  "displayed_balance": 250.0,
  "spendable_balance": 200.0,
  "buckets": [
    {
      "bucket_type": "EarnedCredit",
      "displayed": 150.0,
      "spendable": 150.0,
      "count": 3
    },
    {
      "bucket_type": "CodPending",
      "displayed": 100.0,
      "spendable": 50.0,
      "count": 2
    }
  ]
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 500 | Database error |

---

## Get Historical Balance

**Method:** GET
**Path:** `/wallets/{wallet_id}/balance/at`

Returns the wallet balance at a specific point in time.

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| wallet_id | uuid | Wallet ID |

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| at | string | yes | - | ISO 8601 timestamp to query the balance at |

### Example Request

```bash
curl "http://localhost:3000/wallets/w1a2l3l4-e5t6-7890-abcd-ef1234567890/balance/at?at=2026-03-01T00:00:00Z"
```

### Response

```json
{
  "wallet_id": "w1a2l3l4-e5t6-7890-abcd-ef1234567890",
  "displayed_balance": 100.0,
  "spendable_balance": 100.0,
  "buckets": [
    {
      "bucket_type": "EarnedCredit",
      "displayed": 100.0,
      "spendable": 100.0,
      "count": 2
    }
  ]
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing or invalid `at` parameter |
| 500 | Database error |
