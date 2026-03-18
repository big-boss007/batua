# Notifications API

## Send Notification

**Method:** POST
**Path:** `/notifications/send`

Sends a notification to a customer using a named template. The system resolves the template by name and locale, renders variables, selects an appropriate connector, and dispatches the message.

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| customer_id | uuid | yes | - | Customer to notify |
| template_name | string | yes | - | Template name (e.g. "earn_credit", "cod_released", "gift_card_received") |
| variables | object | yes | - | Template variables for rendering |
| channel_hint | string | no | null | Preferred channel (e.g. "whatsapp", "sms", "email") |
| locale | string | no | null | Locale override (e.g. "en", "hi") |

### Available Template Names

| Name | Trigger | Description |
|------|---------|-------------|
| earn_credit | Credit earned | Customer earned wallet credits |
| redeem_credit | Credit redeemed | Customer redeemed credits on an order |
| cod_pending | COD order placed | Credits held pending delivery |
| cod_released | COD delivered | Held credits released to wallet |
| cod_cancelled | COD RTO/cancelled | Held credits reversed |
| expiry | Credits expiring | Upcoming credit expiry reminder |
| gift_card_received | Gift card received | Customer received a gift card |
| referral_reward | Referral reward | Referral bonus credited |
| tier_upgrade | Tier upgrade | Customer loyalty tier upgraded |

### Example Request

```bash
curl -X POST http://localhost:3000/notifications/send \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "template_name": "earn_credit",
    "variables": {
      "customer_name": "Priya",
      "amount": "74.95",
      "currency": "INR",
      "merchant_name": "Desi Threads",
      "order_id": "#1042"
    },
    "channel_hint": "whatsapp",
    "locale": "en"
  }'
```

### Response

```json
{
  "log_id": "nl12a3b4-c5d6-7890-ef12-345678901234",
  "channel": "whatsapp",
  "status": "sent",
  "external_message_id": "wamid.HBgNOTE5..."
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 404 | Template not found for the given name and locale |
| 404 | No active connector found for the channel |
| 500 | Notification delivery failure or database error |

---

## Create Template

**Method:** POST
**Path:** `/notifications/templates`

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | no | null | Merchant ID (null for global templates) |
| name | string | yes | - | Template name identifier |
| channel | string | yes | - | Channel: "whatsapp", "sms", or "email" |
| locale | string | no | "en" | Locale code |
| template_id | string | no | null | External template ID (e.g. WhatsApp BSP template ID) |
| body_template | string | yes | - | Template body with {{variable}} placeholders |
| variables | object | no | null | JSON schema describing expected variables |

### Example Request

```bash
curl -X POST http://localhost:3000/notifications/templates \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "earn_credit",
    "channel": "whatsapp",
    "locale": "en",
    "template_id": "earn_credit_en_v1",
    "body_template": "Hi {{customer_name}}, you earned {{currency}} {{amount}} on your order {{order_id}} at {{merchant_name}}!",
    "variables": {
      "customer_name": "string",
      "amount": "string",
      "currency": "string",
      "order_id": "string",
      "merchant_name": "string"
    }
  }'
```

### Response

```json
{
  "id": "nt12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "earn_credit",
  "channel": "whatsapp",
  "locale": "en",
  "template_id": "earn_credit_en_v1",
  "body_template": "Hi {{customer_name}}, you earned {{currency}} {{amount}} on your order {{order_id}} at {{merchant_name}}!",
  "variables": {
    "customer_name": "string",
    "amount": "string",
    "currency": "string",
    "order_id": "string",
    "merchant_name": "string"
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
| 409 | Template with same name + channel + locale already exists |
| 500 | Database error |

---

## List Templates

**Method:** GET
**Path:** `/notifications/templates`

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |

### Example Request

```bash
curl "http://localhost:3000/notifications/templates?merchant_id=a1b2c3d4-e5f6-7890-abcd-ef1234567890"
```

### Response

```json
[
  {
    "id": "nt12a3b4-c5d6-7890-ef12-345678901234",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "earn_credit",
    "channel": "whatsapp",
    "locale": "en",
    "template_id": "earn_credit_en_v1",
    "body_template": "Hi {{customer_name}}, you earned {{currency}} {{amount}} on your order {{order_id}} at {{merchant_name}}!",
    "variables": {},
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

## Update Template

**Method:** PUT
**Path:** `/notifications/templates/{id}`

### Path Parameters

| Field | Type | Description |
|-------|------|-------------|
| id | uuid | Template ID |

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| template_id | string | no | - | Updated external template ID |
| body_template | string | no | - | Updated body template |
| variables | object | no | - | Updated variables schema |
| is_active | boolean | no | - | Enable or disable the template |

### Example Request

```bash
curl -X PUT http://localhost:3000/notifications/templates/nt12a3b4-c5d6-7890-ef12-345678901234 \
  -H "Content-Type: application/json" \
  -d '{
    "body_template": "Hi {{customer_name}}, you just earned {{currency}} {{amount}} store credit at {{merchant_name}}! Use it on your next order.",
    "is_active": true
  }'
```

### Response

```json
{
  "id": "nt12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "earn_credit",
  "channel": "whatsapp",
  "locale": "en",
  "template_id": "earn_credit_en_v1",
  "body_template": "Hi {{customer_name}}, you just earned {{currency}} {{amount}} store credit at {{merchant_name}}! Use it on your next order.",
  "variables": {},
  "is_active": true,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:05:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 404 | Template not found |
| 500 | Database error |

---

## Create Connector

**Method:** POST
**Path:** `/notifications/connectors`

Configures a notification delivery connector (e.g. WhatsApp BSP, SMS gateway).

### Request

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | no | null | Merchant ID (null for global connectors) |
| capability | string | yes | - | Connector capability (e.g. "whatsapp-bsp", "sms", "email") |
| vendor | string | yes | - | Vendor name (e.g. "gupshup", "twilio", "msg91") |
| config | object | no | null | Vendor-specific configuration (API keys, endpoints) |
| priority | integer | no | 0 | Priority for connector selection (higher = preferred) |

### Example Request

```bash
curl -X POST http://localhost:3000/notifications/connectors \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "capability": "whatsapp-bsp",
    "vendor": "gupshup",
    "config": {
      "api_key": "gup_...",
      "source_phone": "+919999000000",
      "app_name": "DesiThreadsApp"
    },
    "priority": 10
  }'
```

### Response

```json
{
  "id": "nc12a3b4-c5d6-7890-ef12-345678901234",
  "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "capability": "whatsapp-bsp",
  "vendor": "gupshup",
  "config": {
    "api_key": "gup_...",
    "source_phone": "+919999000000",
    "app_name": "DesiThreadsApp"
  },
  "is_active": true,
  "priority": 10,
  "created_at": "2026-03-18T10:00:00Z",
  "updated_at": "2026-03-18T10:00:00Z"
}
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing required fields |
| 500 | Database error |

---

## List Connectors

**Method:** GET
**Path:** `/notifications/connectors`

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |

### Example Request

```bash
curl "http://localhost:3000/notifications/connectors?merchant_id=a1b2c3d4-e5f6-7890-abcd-ef1234567890"
```

### Response

```json
[
  {
    "id": "nc12a3b4-c5d6-7890-ef12-345678901234",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "capability": "whatsapp-bsp",
    "vendor": "gupshup",
    "config": {},
    "is_active": true,
    "priority": 10,
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

## List Notification Logs

**Method:** GET
**Path:** `/notifications/logs`

Returns the history of sent notifications.

### Query Parameters

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| merchant_id | uuid | yes | - | Merchant ID |
| customer_id | uuid | no | - | Filter by customer |

### Example Request

```bash
curl "http://localhost:3000/notifications/logs?merchant_id=a1b2c3d4-e5f6-7890-abcd-ef1234567890&customer_id=c1d2e3f4-5678-90ab-cdef-123456789012"
```

### Response

```json
[
  {
    "id": "nl12a3b4-c5d6-7890-ef12-345678901234",
    "merchant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "customer_id": "c1d2e3f4-5678-90ab-cdef-123456789012",
    "template_id": "nt12a3b4-c5d6-7890-ef12-345678901234",
    "channel": "whatsapp",
    "variables": {
      "customer_name": "Priya",
      "amount": "74.95",
      "currency": "INR"
    },
    "status": "sent",
    "external_message_id": "wamid.HBgNOTE5...",
    "sent_at": "2026-03-18T10:00:02Z",
    "created_at": "2026-03-18T10:00:00Z"
  }
]
```

### Error Cases

| Status | Condition |
|--------|-----------|
| 400 | Missing merchant_id |
| 500 | Database error |
