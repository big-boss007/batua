ALTER TABLE wallet_policies
    DROP COLUMN IF EXISTS default_conversion_rate,
    DROP COLUMN IF EXISTS is_transferable,
    DROP COLUMN IF EXISTS excluded_collections;
