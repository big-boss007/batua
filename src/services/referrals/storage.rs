use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::types::{
    CodeCreationTrigger, CreateCodeRequest, CreateProgramRequest, ReferralAnalytics, ReferralCode,
    ReferralCodeWithCustomer, ReferralConversion, ReferralProgram,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_program(
    pool: &PgPool,
    req: &CreateProgramRequest,
) -> Result<ReferralProgram, AppError> {
    let trigger = req.code_creation_trigger.clone().unwrap_or_default();
    let trigger_str = match trigger {
        CodeCreationTrigger::OnRegistration => "on_registration",
        CodeCreationTrigger::OnFirstPurchase => "on_first_purchase",
    };

    let program = sqlx::query_as::<_, ReferralProgram>(
        r#"
        INSERT INTO referral_programs (merchant_id, referrer_reward_amount, referee_reward_amount, max_referrals_per_customer, code_creation_trigger)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, merchant_id,
                  referrer_reward_amount::float8 AS referrer_reward_amount,
                  referee_reward_amount::float8 AS referee_reward_amount,
                  referrer_bucket_type, referee_bucket_type,
                  max_referrals_per_customer, is_active, code_creation_trigger, created_at, updated_at
        "#,
    )
    .bind(req.merchant_id)
    .bind(req.referrer_reward_amount)
    .bind(req.referee_reward_amount)
    .bind(req.max_referrals_per_customer)
    .bind(trigger_str)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("referral program already exists for this merchant".to_string())
        }
        other => AppError::Database(other),
    })?;

    Ok(program)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_program(
    pool: &PgPool,
    merchant_id: Uuid,
    req: &super::types::UpdateProgramRequest,
) -> Result<ReferralProgram, AppError> {
    let trigger_str: Option<&str> = req.code_creation_trigger.as_ref().map(|t| match t {
        CodeCreationTrigger::OnRegistration => "on_registration",
        CodeCreationTrigger::OnFirstPurchase => "on_first_purchase",
    });

    let program = sqlx::query_as::<_, ReferralProgram>(
        r#"
        UPDATE referral_programs
        SET referrer_reward_amount = COALESCE($2, referrer_reward_amount),
            referee_reward_amount = COALESCE($3, referee_reward_amount),
            max_referrals_per_customer = CASE WHEN $4 THEN $5 ELSE max_referrals_per_customer END,
            is_active = COALESCE($6, is_active),
            code_creation_trigger = COALESCE($7, code_creation_trigger),
            updated_at = now()
        WHERE merchant_id = $1
        RETURNING id, merchant_id,
                  referrer_reward_amount::float8 AS referrer_reward_amount,
                  referee_reward_amount::float8 AS referee_reward_amount,
                  referrer_bucket_type, referee_bucket_type,
                  max_referrals_per_customer, is_active, code_creation_trigger, created_at, updated_at
        "#,
    )
    .bind(merchant_id)
    .bind(req.referrer_reward_amount)
    .bind(req.referee_reward_amount)
    .bind(req.max_referrals_per_customer.is_some())
    .bind(req.max_referrals_per_customer.flatten())
    .bind(req.is_active)
    .bind(trigger_str)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(program)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_program(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Option<ReferralProgram>, AppError> {
    let program = sqlx::query_as::<_, ReferralProgram>(
        r#"
        SELECT id, merchant_id,
               referrer_reward_amount::float8 AS referrer_reward_amount,
               referee_reward_amount::float8 AS referee_reward_amount,
               referrer_bucket_type, referee_bucket_type,
               max_referrals_per_customer, is_active, code_creation_trigger, created_at, updated_at
        FROM referral_programs
        WHERE merchant_id = $1
        "#,
    )
    .bind(merchant_id)
    .fetch_optional(pool)
    .await?;

    Ok(program)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_referral_code(
    pool: &PgPool,
    req: &CreateCodeRequest,
    generated_code: &str,
) -> Result<ReferralCode, AppError> {
    let code = sqlx::query_as::<_, ReferralCode>(
        r#"
        INSERT INTO referral_codes (merchant_id, customer_id, code, is_vanity, is_creator, commission_rate)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, merchant_id, customer_id, code, is_vanity, is_creator,
                  commission_rate::float8 AS commission_rate,
                  total_referrals, total_conversions, is_active, created_at
        "#,
    )
    .bind(req.merchant_id)
    .bind(req.customer_id)
    .bind(generated_code)
    .bind(req.is_vanity)
    .bind(req.is_creator)
    .bind(req.commission_rate)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!("referral code '{generated_code}' already exists"))
        }
        other => AppError::Database(other),
    })?;

    Ok(code)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_referral_code(pool: &PgPool, code: &str) -> Result<ReferralCode, AppError> {
    let referral_code = sqlx::query_as::<_, ReferralCode>(
        r#"
        SELECT id, merchant_id, customer_id, code, is_vanity, is_creator,
               commission_rate::float8 AS commission_rate,
               total_referrals, total_conversions, is_active, created_at
        FROM referral_codes
        WHERE code = $1
        "#,
    )
    .bind(code)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("referral code '{code}' not found")))?;

    Ok(referral_code)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_customer_referral_code(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<Option<ReferralCode>, AppError> {
    let code = sqlx::query_as::<_, ReferralCode>(
        r#"
        SELECT id, merchant_id, customer_id, code, is_vanity, is_creator,
               commission_rate::float8 AS commission_rate,
               total_referrals, total_conversions, is_active, created_at
        FROM referral_codes
        WHERE merchant_id = $1 AND customer_id = $2
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_optional(pool)
    .await?;

    Ok(code)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn increment_referral_stats(
    pool: &PgPool,
    code_id: Uuid,
    is_conversion: bool,
) -> Result<(), AppError> {
    if is_conversion {
        sqlx::query(
            r#"
            UPDATE referral_codes
            SET total_referrals = total_referrals + 1,
                total_conversions = total_conversions + 1
            WHERE id = $1
            "#,
        )
        .bind(code_id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"
            UPDATE referral_codes
            SET total_referrals = total_referrals + 1
            WHERE id = $1
            "#,
        )
        .bind(code_id)
        .execute(pool)
        .await?;
    }

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_conversion(
    pool: &PgPool,
    merchant_id: Uuid,
    referral_code_id: Uuid,
    referrer_id: Uuid,
    referee_id: Uuid,
    order_id: Option<&str>,
    referrer_entry_id: Option<Uuid>,
    referee_entry_id: Option<Uuid>,
    referee_ip: Option<&str>,
    referee_device_fingerprint: Option<&str>,
    is_suspicious: bool,
    fraud_signals: &serde_json::Value,
) -> Result<ReferralConversion, AppError> {
    let conversion = sqlx::query_as::<_, ReferralConversion>(
        r#"
        INSERT INTO referral_conversions (
            merchant_id, referral_code_id, referrer_id, referee_id,
            order_id, referrer_entry_id, referee_entry_id,
            referee_ip, referee_device_fingerprint,
            is_suspicious, fraud_signals
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, merchant_id, referral_code_id, referrer_id, referee_id,
                  order_id, referrer_entry_id, referee_entry_id,
                  referee_ip, referee_device_fingerprint,
                  is_suspicious, fraud_signals, created_at
        "#,
    )
    .bind(merchant_id)
    .bind(referral_code_id)
    .bind(referrer_id)
    .bind(referee_id)
    .bind(order_id)
    .bind(referrer_entry_id)
    .bind(referee_entry_id)
    .bind(referee_ip)
    .bind(referee_device_fingerprint)
    .bind(is_suspicious)
    .bind(fraud_signals)
    .fetch_one(pool)
    .await?;

    Ok(conversion)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_conversions(
    pool: &PgPool,
    merchant_id: Uuid,
    page: i32,
    limit: i32,
) -> Result<Vec<ReferralConversion>, AppError> {
    let offset = (page - 1) * limit;

    let conversions = sqlx::query_as::<_, ReferralConversion>(
        r#"
        SELECT id, merchant_id, referral_code_id, referrer_id, referee_id,
               order_id, referrer_entry_id, referee_entry_id,
               referee_ip, referee_device_fingerprint,
               is_suspicious, fraud_signals, created_at
        FROM referral_conversions
        WHERE merchant_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(merchant_id)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    Ok(conversions)
}

#[derive(Debug, sqlx::FromRow)]
struct AnalyticsRow {
    total_codes: Option<i64>,
    total_referrals: Option<i64>,
    total_conversions: Option<i64>,
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_referral_analytics(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<ReferralAnalytics, AppError> {
    let code_stats: AnalyticsRow = sqlx::query_as(
        r#"
        SELECT
            COUNT(*)::int8 AS total_codes,
            COALESCE(SUM(total_referrals), 0)::int8 AS total_referrals,
            COALESCE(SUM(total_conversions), 0)::int8 AS total_conversions
        FROM referral_codes
        WHERE merchant_id = $1
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    let suspicious_count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)::int8
        FROM referral_conversions
        WHERE merchant_id = $1 AND is_suspicious = true
        "#,
    )
    .bind(merchant_id)
    .fetch_one(pool)
    .await?;

    let total_referrals = code_stats.total_referrals.unwrap_or(0);
    let total_conversions = code_stats.total_conversions.unwrap_or(0);

    let conversion_rate = if total_referrals > 0 {
        (total_conversions as f64 / total_referrals as f64) * 100.0
    } else {
        0.0
    };

    Ok(ReferralAnalytics {
        total_codes: code_stats.total_codes.unwrap_or(0),
        total_referrals,
        total_conversions,
        total_suspicious: suspicious_count.0,
        conversion_rate,
    })
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn count_customer_referrals(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
) -> Result<i64, AppError> {
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)::int8
        FROM referral_conversions rc
        JOIN referral_codes c ON c.id = rc.referral_code_id
        WHERE c.merchant_id = $1 AND c.customer_id = $2 AND rc.is_suspicious = false
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_merchant_referral_codes(
    pool: &PgPool,
    merchant_id: Uuid,
    page: i32,
    limit: i32,
) -> Result<Vec<ReferralCodeWithCustomer>, AppError> {
    let offset = (page - 1) * limit;

    let codes = sqlx::query_as::<_, ReferralCodeWithCustomer>(
        r#"
        SELECT rc.id, rc.merchant_id, rc.customer_id, rc.code, rc.is_vanity, rc.is_creator,
               rc.commission_rate::float8 AS commission_rate,
               rc.total_referrals, rc.total_conversions, rc.is_active, rc.created_at,
               c.phone AS customer_phone, c.name AS customer_name
        FROM referral_codes rc
        LEFT JOIN customers c ON c.id = rc.customer_id
        WHERE rc.merchant_id = $1
        ORDER BY rc.total_conversions DESC, rc.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(merchant_id)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    Ok(codes)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_active_program_trigger(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Option<CodeCreationTrigger>, AppError> {
    let row: Option<(CodeCreationTrigger,)> = sqlx::query_as(
        r#"
        SELECT code_creation_trigger
        FROM referral_programs
        WHERE merchant_id = $1 AND is_active = true
        "#,
    )
    .bind(merchant_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0))
}
