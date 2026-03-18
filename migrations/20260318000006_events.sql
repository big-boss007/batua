CREATE TABLE events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    event_type TEXT NOT NULL,  -- e.g., "order.completed", "order.refunded"
    event_source TEXT NOT NULL,  -- e.g., "shopify", "shiprocket"
    external_event_id TEXT NOT NULL,  -- Source system's event ID
    payload JSONB NOT NULL,
    state event_state NOT NULL DEFAULT 'received',
    idempotency_key TEXT NOT NULL UNIQUE,  -- Prevents duplicate event processing
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at TIMESTAMPTZ
);

CREATE INDEX idx_events_merchant ON events(merchant_id);
CREATE INDEX idx_events_type ON events(event_type);
CREATE INDEX idx_events_state ON events(state);
CREATE INDEX idx_events_idempotency ON events(idempotency_key);
CREATE INDEX idx_events_external ON events(merchant_id, external_event_id);
