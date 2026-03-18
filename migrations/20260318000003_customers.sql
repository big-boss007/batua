CREATE TABLE customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phone TEXT NOT NULL,  -- E.164 normalized
    email TEXT,
    name TEXT,
    external_id TEXT,  -- Shopify customer ID
    is_verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(phone)
);

CREATE INDEX idx_customers_phone ON customers(phone);
CREATE INDEX idx_customers_external_id ON customers(external_id);
