-- Rework memberships: a membership is a loyalty tier assigned manually with expiry.
-- Replace plan_id (refs membership_plans) with tier_id (refs loyalty_tiers).

ALTER TABLE customer_memberships DROP CONSTRAINT IF EXISTS customer_memberships_plan_id_fkey;
ALTER TABLE customer_memberships DROP CONSTRAINT IF EXISTS customer_memberships_merchant_id_customer_id_plan_id_key;

ALTER TABLE customer_memberships RENAME COLUMN plan_id TO tier_id;

ALTER TABLE customer_memberships
    ADD CONSTRAINT customer_memberships_tier_id_fkey
    FOREIGN KEY (tier_id) REFERENCES loyalty_tiers(id);

ALTER TABLE customer_memberships
    ADD CONSTRAINT customer_memberships_merchant_customer_tier_unique
    UNIQUE (merchant_id, customer_id, tier_id);

DROP TABLE IF EXISTS membership_plans;
