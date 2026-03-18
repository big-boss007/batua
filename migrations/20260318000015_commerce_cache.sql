-- Pre-synced commerce data for rule evaluation (Category B enrichment)
CREATE TABLE product_collection_mappings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    product_id TEXT NOT NULL,  -- Shopify product ID
    collection_id TEXT NOT NULL,  -- Shopify collection ID
    collection_name TEXT NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, product_id, collection_id)
);

CREATE INDEX idx_pcm_merchant_product ON product_collection_mappings(merchant_id, product_id);

CREATE TABLE customer_order_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    total_orders INTEGER NOT NULL DEFAULT 0,
    total_spend DOUBLE PRECISION NOT NULL DEFAULT 0,
    first_order_at TIMESTAMPTZ,
    last_order_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, customer_id)
);

CREATE INDEX idx_cos_merchant_customer ON customer_order_stats(merchant_id, customer_id);
