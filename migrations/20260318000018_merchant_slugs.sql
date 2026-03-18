ALTER TABLE merchants ADD COLUMN slug TEXT UNIQUE;
CREATE INDEX idx_merchants_slug ON merchants(slug) WHERE slug IS NOT NULL;
