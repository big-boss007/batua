-- Connector abstraction: capability -> vendor mapping
CREATE TABLE connectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID REFERENCES merchants(id),  -- NULL = system default
    capability TEXT NOT NULL,  -- 'whatsapp-bsp', 'sms', 'email', 'payment-gateway'
    vendor TEXT NOT NULL,  -- 'interakt', 'wati', 'razorpay'
    config JSONB NOT NULL DEFAULT '{}',  -- Vendor-specific config (encrypted sensitive fields)
    is_active BOOLEAN NOT NULL DEFAULT true,
    priority INTEGER NOT NULL DEFAULT 0,  -- For fallback ordering
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_connectors_merchant ON connectors(merchant_id);
CREATE INDEX idx_connectors_capability ON connectors(capability);
CREATE UNIQUE INDEX idx_connectors_unique ON connectors(merchant_id, capability, vendor);
