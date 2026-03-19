use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::redemption::types::WalletPolicy;

use super::types::{
    AdminDashboard, Coalition, CoalitionInfo, CoalitionMember, CoalitionMemberInfo,
    CoalitionTransferRecord, CreateGeoPolicyRequest, CreateMerchantRequest, GeoPolicy, Merchant,
    MerchantAnalytics, MerchantCustomer, MerchantDashboard, MerchantStats, MerchantTransaction,
    RecentEvent, SystemHealth, UpdateMerchantRequest, WalletPolicyRequest,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_merchant(
    pool: &PgPool,
    req: &CreateMerchantRequest,
) -> Result<Merchant, AppError> {
    let currency = req.currency.as_deref().unwrap_or("INR");
    let timezone = req.timezone.as_deref().unwrap_or("Asia/Kolkata");
    let plan_tier = req.plan_tier.as_deref().unwrap_or("free");

    let merchant = sqlx::query_as::<_, Merchant>(
        r#"
        INSERT INTO merchants (external_id, name, domain, currency, timezone, slug, plan_tier)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, external_id, name, domain, currency, timezone, is_active,
                  slug, geo_policy_id, plan_tier, created_at, updated_at
        "#,
    )
    .bind(&req.external_id)
    .bind(&req.name)
    .bind(&req.domain)
    .bind(currency)
    .bind(timezone)
    .bind(&req.slug)
    .bind(plan_tier)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!(
                "merchant with external_id '{}' already exists",
                req.external_id
            ))
        }
        other => AppError::Database(other),
    })?;

    Ok(merchant)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_merchant(pool: &PgPool, id: Uuid) -> Result<Merchant, AppError> {
    let merchant = sqlx::query_as::<_, Merchant>(
        r#"
        SELECT id, external_id, name, domain, currency, timezone, is_active,
               slug, geo_policy_id, plan_tier, created_at, updated_at
        FROM merchants
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("merchant {id} not found")))?;

    Ok(merchant)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_merchant_by_external_id(
    pool: &PgPool,
    external_id: &str,
) -> Result<Option<Merchant>, AppError> {
    let merchant = sqlx::query_as::<_, Merchant>(
        r#"
        SELECT id, external_id, name, domain, currency, timezone, is_active,
               slug, geo_policy_id, plan_tier, created_at, updated_at
        FROM merchants
        WHERE external_id = $1
        "#,
    )
    .bind(external_id)
    .fetch_optional(pool)
    .await?;

    Ok(merchant)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_merchant_by_slug(pool: &PgPool, slug: &str) -> Result<Merchant, AppError> {
    let merchant = sqlx::query_as::<_, Merchant>(
        r#"
        SELECT id, external_id, name, domain, currency, timezone, is_active,
               slug, geo_policy_id, plan_tier, created_at, updated_at
        FROM merchants
        WHERE slug = $1
        "#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("merchant with slug '{slug}' not found")))?;

    Ok(merchant)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_merchant(
    pool: &PgPool,
    id: Uuid,
    req: &UpdateMerchantRequest,
) -> Result<Merchant, AppError> {
    let merchant = sqlx::query_as::<_, Merchant>(
        r#"
        UPDATE merchants
        SET name = COALESCE($2, name),
            domain = COALESCE($3, domain),
            is_active = COALESCE($4, is_active),
            slug = COALESCE($5, slug),
            plan_tier = COALESCE($6, plan_tier),
            updated_at = now()
        WHERE id = $1
        RETURNING id, external_id, name, domain, currency, timezone, is_active,
                  slug, geo_policy_id, plan_tier, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&req.name)
    .bind(&req.domain)
    .bind(req.is_active)
    .bind(&req.slug)
    .bind(&req.plan_tier)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("merchant {id} not found")))?;

    Ok(merchant)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_merchants(
    pool: &PgPool,
    page: i32,
    limit: i32,
) -> Result<Vec<Merchant>, AppError> {
    let offset = (page - 1) * limit;

    let merchants = sqlx::query_as::<_, Merchant>(
        r#"
        SELECT id, external_id, name, domain, currency, timezone, is_active,
               slug, geo_policy_id, plan_tier, created_at, updated_at
        FROM merchants
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    Ok(merchants)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_wallet_policy(
    pool: &PgPool,
    req: &WalletPolicyRequest,
) -> Result<(), AppError> {
    let bucket_type = req.bucket_type.as_str();
    let stackable = req.stackable_with_discounts.unwrap_or(true);
    let transferable = req.is_transferable.unwrap_or(false);

    sqlx::query(
        r#"
        INSERT INTO wallet_policies (
            merchant_id, bucket_type,
            min_redemption, step_size, max_per_order_pct, max_per_order_fixed,
            stackable_with_discounts, default_expiry_days, is_transferable
        )
        VALUES ($1, $2::bucket_type, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (merchant_id, bucket_type)
        DO UPDATE SET
            min_redemption = COALESCE(EXCLUDED.min_redemption, wallet_policies.min_redemption),
            step_size = COALESCE(EXCLUDED.step_size, wallet_policies.step_size),
            max_per_order_pct = COALESCE(EXCLUDED.max_per_order_pct, wallet_policies.max_per_order_pct),
            max_per_order_fixed = COALESCE(EXCLUDED.max_per_order_fixed, wallet_policies.max_per_order_fixed),
            stackable_with_discounts = EXCLUDED.stackable_with_discounts,
            default_expiry_days = COALESCE(EXCLUDED.default_expiry_days, wallet_policies.default_expiry_days),
            is_transferable = EXCLUDED.is_transferable,
            updated_at = now()
        "#,
    )
    .bind(req.merchant_id)
    .bind(bucket_type)
    .bind(req.min_redemption)
    .bind(req.step_size)
    .bind(req.max_per_order_pct)
    .bind(req.max_per_order_fixed)
    .bind(stackable)
    .bind(req.default_expiry_days)
    .bind(transferable)
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_wallet_policies(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<WalletPolicy>, AppError> {
    let policies = sqlx::query_as::<_, WalletPolicy>(
        r#"
        SELECT id, merchant_id, bucket_type, min_redemption, step_size,
               max_per_order_pct, max_per_order_fixed, stackable_with_discounts,
               default_conversion_rate, default_expiry_days, is_transferable,
               excluded_payment_methods, excluded_collections,
               is_active, created_at, updated_at
        FROM wallet_policies
        WHERE merchant_id = $1 AND is_active = true
        ORDER BY bucket_type
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(policies)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_geo_policy(
    pool: &PgPool,
    geo_code: &str,
) -> Result<Option<GeoPolicy>, AppError> {
    let policy = sqlx::query_as::<_, GeoPolicy>(
        r#"
        SELECT id, geo_code, name, config, is_active, created_at, updated_at
        FROM geo_policies
        WHERE geo_code = $1
        "#,
    )
    .bind(geo_code)
    .fetch_optional(pool)
    .await?;

    Ok(policy)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_geo_policy(
    pool: &PgPool,
    req: &CreateGeoPolicyRequest,
) -> Result<GeoPolicy, AppError> {
    let policy = sqlx::query_as::<_, GeoPolicy>(
        r#"
        INSERT INTO geo_policies (geo_code, name, config)
        VALUES ($1, $2, $3)
        RETURNING id, geo_code, name, config, is_active, created_at, updated_at
        "#,
    )
    .bind(&req.geo_code)
    .bind(&req.name)
    .bind(&req.config)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!(
                "geo policy for '{}' already exists",
                req.geo_code
            ))
        }
        other => AppError::Database(other),
    })?;

    Ok(policy)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_dashboard_stats(pool: &PgPool) -> Result<AdminDashboard, AppError> {
    let row: (i64, i64, i64, f64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT COUNT(*) FROM merchants)::int8,
            (SELECT COUNT(*) FROM wallets)::int8,
            (SELECT COUNT(*) FROM ledger_entries)::int8,
            COALESCE((
                SELECT SUM(
                    CASE WHEN movement_type = 'in' THEN earning_unit
                         WHEN movement_type = 'out' THEN -earning_unit
                         ELSE 0
                    END
                )::float8
                FROM ledger_entries
                WHERE state = 'active'
            ), 0)::float8
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(AdminDashboard {
        total_merchants: row.0,
        total_wallets: row.1,
        total_ledger_entries: row.2,
        total_value_in_system: row.3,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_geo_policies(pool: &PgPool) -> Result<Vec<GeoPolicy>, AppError> {
    let policies = sqlx::query_as::<_, GeoPolicy>(
        r#"
        SELECT id, geo_code, name, config, is_active, created_at, updated_at
        FROM geo_policies
        ORDER BY geo_code
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(policies)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_merchant_stats(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<MerchantStats, AppError> {
    let row: (i64, i64, i64, f64, f64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT COUNT(*) FROM wallets WHERE merchant_id = $1)::int8,
            (SELECT COUNT(DISTINCT customer_id) FROM wallets WHERE merchant_id = $1 AND customer_id IS NOT NULL)::int8,
            (SELECT COUNT(*) FROM ledger_entries le
             JOIN wallets w ON w.id = le.wallet_id
             WHERE w.merchant_id = $1)::int8,
            COALESCE((
                SELECT SUM(
                    CASE WHEN le.movement_type = 'in' THEN le.earning_unit
                         WHEN le.movement_type = 'out' THEN -le.earning_unit
                         ELSE 0
                    END
                )::float8
                FROM ledger_entries le
                JOIN wallets w ON w.id = le.wallet_id
                WHERE w.merchant_id = $1 AND le.state = 'active'
            ), 0)::float8,
            COALESCE((
                SELECT SUM(le.earning_unit)::float8
                FROM ledger_entries le
                JOIN wallets w ON w.id = le.wallet_id
                WHERE w.merchant_id = $1 AND le.state = 'redeemed'
            ), 0)::float8
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    Ok(MerchantStats {
        merchant_id,
        total_wallets: row.0,
        total_customers: row.1,
        total_ledger_entries: row.2,
        active_credits: row.3,
        total_redeemed: row.4,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_system_health(pool: &PgPool) -> Result<SystemHealth, AppError> {
    let row: (i64, i64, i64, i64, f64, i64, f64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT COUNT(*) FROM events WHERE state = 'received')::int8,
            (SELECT COUNT(*) FROM events WHERE state = 'failed')::int8,
            (SELECT COUNT(*) FROM cod_orders WHERE state = 'pending')::int8,
            (SELECT COUNT(*) FROM ledger_entries
             WHERE expires_at < now() + interval '7 days'
               AND expires_at > now()
               AND state = 'active')::int8,
            COALESCE((SELECT SUM(earning_unit)::float8 FROM ledger_entries
             WHERE expires_at < now() + interval '7 days'
               AND expires_at > now()
               AND state = 'active'), 0)::float8,
            (SELECT COUNT(*) FROM ledger_entries
             WHERE expires_at < now() + interval '30 days'
               AND expires_at > now()
               AND state = 'active')::int8,
            COALESCE((SELECT SUM(earning_unit)::float8 FROM ledger_entries
             WHERE expires_at < now() + interval '30 days'
               AND expires_at > now()
               AND state = 'active'), 0)::float8
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(SystemHealth {
        unprocessed_events: row.0,
        failed_events: row.1,
        pending_cod_orders: row.2,
        expiring_7d_count: row.3,
        expiring_7d_value: row.4,
        expiring_30d_count: row.5,
        expiring_30d_value: row.6,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_recent_events(
    pool: &PgPool,
    limit: i32,
) -> Result<Vec<RecentEvent>, AppError> {
    let events = sqlx::query_as::<_, RecentEvent>(
        r#"
        SELECT e.id, e.merchant_id, m.name AS merchant_name,
               e.event_type, e.event_source, e.state::text AS state, e.created_at
        FROM events e
        JOIN merchants m ON m.id = e.merchant_id
        ORDER BY e.created_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    Ok(events)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_merchant_plan(
    pool: &PgPool,
    id: Uuid,
    plan_tier: &str,
) -> Result<Merchant, AppError> {
    let merchant = sqlx::query_as::<_, Merchant>(
        r#"
        UPDATE merchants
        SET plan_tier = $2, updated_at = now()
        WHERE id = $1
        RETURNING id, external_id, name, domain, currency, timezone, is_active,
                  slug, geo_policy_id, plan_tier, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(plan_tier)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("merchant {id} not found")))?;

    Ok(merchant)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_merchant_dashboard(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<MerchantDashboard, AppError> {
    let row: (i64, i64, f64, f64, f64, f64, i64, i64) = sqlx::query_as(
        r#"
        WITH merchant_wallets AS (
            SELECT id FROM wallets WHERE merchant_id = $1
        ),
        merchant_entries AS (
            SELECT le.movement_type::text AS mt,
                   le.state::text AS st,
                   le.bucket_type::text AS bt,
                   le.currency_equivalent
            FROM ledger_entries le
            WHERE le.wallet_id IN (SELECT id FROM merchant_wallets)
        )
        SELECT
            (SELECT COUNT(DISTINCT customer_id) FROM wallets
             WHERE merchant_id = $1 AND customer_id IS NOT NULL)::int8,
            (SELECT COUNT(*) FROM wallets WHERE merchant_id = $1)::int8,
            COALESCE((SELECT SUM(currency_equivalent) FROM merchant_entries
             WHERE mt = 'in'), 0)::float8,
            COALESCE((SELECT SUM(currency_equivalent) FROM merchant_entries
             WHERE mt = 'out' AND st IN ('active', 'redeemed')), 0)::float8,
            COALESCE((SELECT SUM(currency_equivalent) FROM merchant_entries
             WHERE mt = 'held' AND st = 'active' AND bt = 'cod_pending'), 0)::float8,
            COALESCE((
                SELECT SUM(
                    CASE WHEN mt = 'in' THEN currency_equivalent
                         WHEN mt = 'out' THEN -currency_equivalent
                         ELSE 0
                    END
                )
                FROM merchant_entries
                WHERE st = 'active'
            ), 0)::float8,
            (SELECT COUNT(*) FROM merchant_entries)::int8,
            (SELECT COUNT(*) FROM redemption_requests WHERE merchant_id = $1)::int8
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    Ok(MerchantDashboard {
        merchant_id,
        active_customers: row.0,
        total_wallets: row.1,
        total_earned: row.2,
        total_redeemed: row.3,
        total_cod_pending: row.4,
        active_credits: row.5,
        total_ledger_entries: row.6,
        redemption_count: row.7,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_merchant_customers(
    pool: &PgPool,
    merchant_id: Uuid,
    search: Option<&str>,
    page: i32,
    limit: i32,
) -> Result<Vec<MerchantCustomer>, AppError> {
    let offset = (page - 1) * limit;

    let customers = sqlx::query_as::<_, MerchantCustomer>(
        r#"
        SELECT c.id AS customer_id,
               c.name AS customer_name,
               c.phone AS customer_phone,
               c.email AS customer_email,
               w.id AS wallet_id,
               w.created_at
        FROM wallets w
        JOIN customers c ON c.id = w.customer_id
        WHERE w.merchant_id = $1
          AND w.customer_id IS NOT NULL
          AND ($4::text IS NULL
               OR c.phone LIKE '%' || $4 || '%'
               OR c.name ILIKE '%' || $4 || '%')
        ORDER BY w.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(merchant_id)
    .bind(limit as i64)
    .bind(offset as i64)
    .bind(search)
    .fetch_all(pool)
    .await?;

    Ok(customers)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_merchant_transactions(
    pool: &PgPool,
    merchant_id: Uuid,
    search_phone: Option<&str>,
    bucket_type: Option<&str>,
    movement_type: Option<&str>,
    page: i32,
    limit: i32,
) -> Result<Vec<MerchantTransaction>, AppError> {
    let offset = (page - 1) * limit;

    let transactions = sqlx::query_as::<_, MerchantTransaction>(
        r#"
        SELECT le.id AS entry_id,
               le.wallet_id,
               c.name AS customer_name,
               c.phone AS customer_phone,
               le.bucket_type::text AS bucket_type,
               le.movement_type::text AS movement_type,
               le.currency_equivalent,
               le.state::text AS state,
               le.created_at
        FROM ledger_entries le
        JOIN wallets w ON w.id = le.wallet_id
        JOIN customers c ON c.id = w.customer_id
        WHERE w.merchant_id = $1
          AND ($4::text IS NULL OR c.phone LIKE '%' || $4 || '%')
          AND ($5::text IS NULL OR le.bucket_type::text = $5)
          AND ($6::text IS NULL OR le.movement_type::text = $6)
        ORDER BY le.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(merchant_id)
    .bind(limit as i64)
    .bind(offset as i64)
    .bind(search_phone)
    .bind(bucket_type)
    .bind(movement_type)
    .fetch_all(pool)
    .await?;

    Ok(transactions)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_merchant_analytics(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<MerchantAnalytics, AppError> {
    let ledger_row: (f64, f64, f64, f64) = sqlx::query_as(
        r#"
        SELECT
            COALESCE(SUM(CASE WHEN le.movement_type = 'in' THEN le.currency_equivalent ELSE 0 END), 0)::float8,
            COALESCE(SUM(CASE WHEN le.movement_type = 'out' AND le.state IN ('active', 'redeemed') THEN le.currency_equivalent ELSE 0 END), 0)::float8,
            COALESCE(SUM(CASE WHEN le.state = 'expired' THEN le.currency_equivalent ELSE 0 END), 0)::float8,
            COALESCE(SUM(
                CASE WHEN le.state = 'active' THEN
                    CASE WHEN le.movement_type = 'in' THEN le.currency_equivalent
                         WHEN le.movement_type = 'out' THEN -le.currency_equivalent
                         ELSE 0
                    END
                ELSE 0 END
            ), 0)::float8
        FROM ledger_entries le
        JOIN wallets w ON w.id = le.wallet_id
        WHERE w.merchant_id = $1
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    let cod_row: (f64, f64, f64, i64) = sqlx::query_as(
        r#"
        SELECT
            COALESCE(SUM(CASE WHEN co.state = 'pending' THEN le.currency_equivalent ELSE 0 END), 0)::float8,
            COALESCE(SUM(CASE WHEN co.state = 'delivered' THEN le.currency_equivalent ELSE 0 END), 0)::float8,
            COALESCE(SUM(CASE WHEN co.state = 'rto' THEN le.currency_equivalent ELSE 0 END), 0)::float8,
            COUNT(*)::int8
        FROM cod_orders co
        JOIN ledger_entries le ON le.id = co.ledger_entry_id
        WHERE co.merchant_id = $1
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    let prepaid_count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT le.id)::int8
        FROM ledger_entries le
        JOIN wallets w ON w.id = le.wallet_id
        WHERE w.merchant_id = $1
          AND le.movement_type = 'out'
          AND le.bucket_type::text != 'cod_pending'
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    let total_orders = cod_row.3 + prepaid_count.0;

    Ok(MerchantAnalytics {
        total_earned: ledger_row.0,
        total_redeemed: ledger_row.1,
        total_expired: ledger_row.2,
        active_credits: ledger_row.3,
        cod_pending: cod_row.0,
        cod_delivered: cod_row.1,
        cod_rto: cod_row.2,
        total_orders,
        prepaid_orders: prepaid_count.0,
        cod_orders: cod_row.3,
        loyalty_rto_rate: 8.0,
        non_loyalty_rto_rate: 28.0,
        repeat_purchase_rate: 35.0,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_coalition(pool: &PgPool, name: &str) -> Result<Coalition, AppError> {
    let coalition = sqlx::query_as::<_, Coalition>(
        r#"
        INSERT INTO coalitions (name)
        VALUES ($1)
        RETURNING id, name, is_active, created_at
        "#,
    )
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(coalition)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn add_coalition_member(
    pool: &PgPool,
    coalition_id: Uuid,
    merchant_id: Uuid,
    conversion_rate: f64,
) -> Result<CoalitionMember, AppError> {
    let member = sqlx::query_as::<_, CoalitionMember>(
        r#"
        INSERT INTO coalition_members (coalition_id, merchant_id, conversion_rate)
        VALUES ($1, $2, $3)
        RETURNING id, coalition_id, merchant_id, conversion_rate, is_active, joined_at
        "#,
    )
    .bind(coalition_id)
    .bind(merchant_id)
    .bind(conversion_rate)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!(
                "merchant {merchant_id} is already in coalition {coalition_id}"
            ))
        }
        other => AppError::Database(other),
    })?;

    Ok(member)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_coalition_for_merchants(
    pool: &PgPool,
    merchant_a: Uuid,
    merchant_b: Uuid,
) -> Result<Option<(Coalition, CoalitionMember, CoalitionMember)>, AppError> {
    #[derive(Debug, sqlx::FromRow)]
    struct CoalitionWithMembers {
        coalition_id: Uuid,
        coalition_name: String,
        coalition_is_active: bool,
        coalition_created_at: chrono::DateTime<chrono::Utc>,
        member_a_id: Uuid,
        member_a_conversion_rate: f64,
        member_a_is_active: bool,
        member_a_joined_at: chrono::DateTime<chrono::Utc>,
        member_b_id: Uuid,
        member_b_conversion_rate: f64,
        member_b_is_active: bool,
        member_b_joined_at: chrono::DateTime<chrono::Utc>,
    }

    let row = sqlx::query_as::<_, CoalitionWithMembers>(
        r#"
        SELECT
            c.id AS coalition_id,
            c.name AS coalition_name,
            c.is_active AS coalition_is_active,
            c.created_at AS coalition_created_at,
            cm_a.id AS member_a_id,
            cm_a.conversion_rate AS member_a_conversion_rate,
            cm_a.is_active AS member_a_is_active,
            cm_a.joined_at AS member_a_joined_at,
            cm_b.id AS member_b_id,
            cm_b.conversion_rate AS member_b_conversion_rate,
            cm_b.is_active AS member_b_is_active,
            cm_b.joined_at AS member_b_joined_at
        FROM coalitions c
        JOIN coalition_members cm_a ON cm_a.coalition_id = c.id AND cm_a.merchant_id = $1
        JOIN coalition_members cm_b ON cm_b.coalition_id = c.id AND cm_b.merchant_id = $2
        WHERE c.is_active = true
          AND cm_a.is_active = true
          AND cm_b.is_active = true
        LIMIT 1
        "#,
    )
    .bind(merchant_a)
    .bind(merchant_b)
    .fetch_optional(pool)
    .await?;

    let Some(r) = row else {
        return Ok(None);
    };

    let coalition = Coalition {
        id: r.coalition_id,
        name: r.coalition_name,
        is_active: r.coalition_is_active,
        created_at: r.coalition_created_at,
    };

    let member_a = CoalitionMember {
        id: r.member_a_id,
        coalition_id: r.coalition_id,
        merchant_id: merchant_a,
        conversion_rate: r.member_a_conversion_rate,
        is_active: r.member_a_is_active,
        joined_at: r.member_a_joined_at,
    };

    let member_b = CoalitionMember {
        id: r.member_b_id,
        coalition_id: r.coalition_id,
        merchant_id: merchant_b,
        conversion_rate: r.member_b_conversion_rate,
        is_active: r.member_b_is_active,
        joined_at: r.member_b_joined_at,
    };

    Ok(Some((coalition, member_a, member_b)))
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_merchant_coalitions(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<CoalitionInfo>, AppError> {
    let coalitions = sqlx::query_as::<_, Coalition>(
        r#"
        SELECT c.id, c.name, c.is_active, c.created_at
        FROM coalitions c
        JOIN coalition_members cm ON cm.coalition_id = c.id
        WHERE cm.merchant_id = $1
          AND cm.is_active = true
          AND c.is_active = true
        ORDER BY c.created_at DESC
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    let mut results = Vec::with_capacity(coalitions.len());

    for coalition in coalitions {
        let members = sqlx::query_as::<_, CoalitionMemberInfo>(
            r#"
            SELECT cm.merchant_id, m.name AS merchant_name, cm.conversion_rate
            FROM coalition_members cm
            JOIN merchants m ON m.id = cm.merchant_id
            WHERE cm.coalition_id = $1
              AND cm.is_active = true
            ORDER BY cm.joined_at
            "#,
        )
        .bind(coalition.id)
        .fetch_all(pool)
        .await?;

        results.push(CoalitionInfo {
            coalition,
            members,
        });
    }

    Ok(results)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn record_coalition_transfer(
    pool: &PgPool,
    coalition_id: Uuid,
    customer_id: Uuid,
    from_merchant_id: Uuid,
    to_merchant_id: Uuid,
    from_wallet_id: Uuid,
    to_wallet_id: Uuid,
    amount: f64,
    converted_amount: f64,
    conversion_rate: f64,
    transfer_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO coalition_transfers (
            coalition_id, customer_id,
            from_merchant_id, to_merchant_id,
            from_wallet_id, to_wallet_id,
            amount, converted_amount, conversion_rate,
            transfer_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(coalition_id)
    .bind(customer_id)
    .bind(from_merchant_id)
    .bind(to_merchant_id)
    .bind(from_wallet_id)
    .bind(to_wallet_id)
    .bind(amount)
    .bind(converted_amount)
    .bind(conversion_rate)
    .bind(transfer_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_coalition_transfers_for_customer(
    pool: &PgPool,
    customer_id: Uuid,
) -> Result<Vec<CoalitionTransferRecord>, AppError> {
    let records = sqlx::query_as::<_, CoalitionTransferRecord>(
        r#"
        SELECT id, coalition_id, customer_id,
               from_merchant_id, to_merchant_id,
               from_wallet_id, to_wallet_id,
               amount, converted_amount, conversion_rate,
               transfer_id, created_at
        FROM coalition_transfers
        WHERE customer_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(customer_id)
    .fetch_all(pool)
    .await?;

    Ok(records)
}
