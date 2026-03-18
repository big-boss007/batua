CREATE TABLE notification_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID REFERENCES merchants(id),
    name TEXT NOT NULL,
    channel TEXT NOT NULL,  -- 'whatsapp', 'sms', 'email'
    locale TEXT NOT NULL DEFAULT 'en',
    template_id TEXT,  -- External template ID (WhatsApp BSP template ID)
    body_template TEXT NOT NULL,
    variables JSONB NOT NULL DEFAULT '[]',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE notification_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    template_id UUID NOT NULL REFERENCES notification_templates(id),
    channel TEXT NOT NULL,
    variables JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',  -- pending, sent, delivered, failed
    external_message_id TEXT,
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_notification_logs_merchant ON notification_logs(merchant_id);
CREATE INDEX idx_notification_logs_customer ON notification_logs(customer_id);
