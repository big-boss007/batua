# Phase 5: Database Migrations

## `20260318000002_merchants.sql` (from Phase 0, extended)
```sql
CREATE TABLE merchants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    external_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    domain TEXT,
    currency TEXT NOT NULL DEFAULT 'INR',
    timezone TEXT NOT NULL DEFAULT 'Asia/Kolkata',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

## `20260318000007_rules_and_snapshots.sql`
- **rules** -- id, merchant_id (FK), rule_type, name, config (JSONB), version (default 1), is_active. Indexes: merchant, merchant+rule_type, merchant+is_active (partial).
- **rule_snapshots** -- id, rule_id (FK), version, config (JSONB frozen). Index: rule_id.
- **campaigns** -- id, merchant_id (FK), name, campaign_type, config (JSONB), base_rule_id (FK rules), multiplier, starts_at, ends_at, is_active. Indexes: merchant, merchant+active+dates.
- **campaign_snapshots** -- id, campaign_id (FK), config, multiplier. Index: campaign_id.

## `20260318000010_connectors.sql`
- **connectors** -- id, merchant_id (FK, nullable for system defaults), capability, vendor, config (JSONB), is_active, priority. Unique index: (merchant_id, capability, vendor).

## `20260318000011_notifications.sql`
- **notification_templates** -- id, merchant_id (FK), name, channel, locale (default 'en'), template_id (external BSP ID), body_template, variables (JSONB), is_active.
- **notification_logs** -- id, merchant_id (FK), customer_id (FK), template_id (FK), channel, variables (JSONB), status (pending/sent/delivered/failed), external_message_id, sent_at. Indexes: merchant_id, customer_id.

## `20260318000016_geo_policies.sql`
- **geo_policies** -- id, geo_code (UNIQUE), name, config (JSONB), is_active.
- Adds `geo_policy_id UUID REFERENCES geo_policies(id)` to merchants table.

## `20260318000018_merchant_slugs.sql`
- Adds `slug TEXT UNIQUE` to merchants table with partial index on non-null values.

## `20260318000019_plan_tier.sql`
- Adds `plan_tier TEXT NOT NULL DEFAULT 'free'` to merchants table.
