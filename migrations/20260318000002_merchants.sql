CREATE TABLE merchants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    external_id TEXT NOT NULL UNIQUE,  -- Shopify shop ID
    name TEXT NOT NULL,
    domain TEXT,
    currency TEXT NOT NULL DEFAULT 'INR',
    timezone TEXT NOT NULL DEFAULT 'Asia/Kolkata',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_merchants_external_id ON merchants(external_id);
