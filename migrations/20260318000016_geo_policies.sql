CREATE TABLE geo_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    geo_code TEXT NOT NULL UNIQUE,  -- 'india', 'indonesia', etc.
    name TEXT NOT NULL,
    config JSONB NOT NULL,  -- All geo-specific behavior
    -- Example for india: {"cod_enabled": true, "default_currency": "INR", "whatsapp_default": true, "upi_topup_enabled": true}
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Link merchants to geo policies
ALTER TABLE merchants ADD COLUMN geo_policy_id UUID REFERENCES geo_policies(id);
