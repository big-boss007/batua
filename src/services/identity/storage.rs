use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::helpers::normalize_phone;
use super::types::{Customer, CustomerQuery, ResolveIdentityRequest, UpdateCustomerRequest};

#[derive(Debug, sqlx::FromRow)]
pub struct CustomerWithWallet {
    pub id: Uuid,
    pub phone: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub external_id: Option<String>,
    pub is_verified: bool,
    pub birthday: Option<chrono::NaiveDate>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub wallet_id: Uuid,
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn resolve_by_phone(
    pool: &PgPool,
    phone: &str,
) -> Result<Option<Customer>, AppError> {
    let normalized = normalize_phone(phone)?;

    let customer = sqlx::query_as::<_, Customer>(
        r#"
        SELECT id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
        FROM customers
        WHERE phone = $1
        "#,
    )
    .bind(&normalized)
    .fetch_optional(pool)
    .await?;

    Ok(customer)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_customer(
    pool: &PgPool,
    req: &ResolveIdentityRequest,
) -> Result<Customer, AppError> {
    let normalized_phone = normalize_phone(&req.phone)?;

    if let Some(ref email) = req.email {
        if !super::helpers::validate_email(email) {
            return Err(AppError::BadRequest(format!("invalid email: {email}")));
        }
    }

    let customer = sqlx::query_as::<_, Customer>(
        r#"
        INSERT INTO customers (phone, email, name, external_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
        "#,
    )
    .bind(&normalized_phone)
    .bind(&req.email)
    .bind(&req.name)
    .bind(&req.external_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!(
                "customer with phone {normalized_phone} already exists"
            ))
        }
        other => AppError::Database(other),
    })?;

    Ok(customer)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_customer(pool: &PgPool, id: Uuid) -> Result<Customer, AppError> {
    let customer = sqlx::query_as::<_, Customer>(
        r#"
        SELECT id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
        FROM customers
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("customer {id} not found")))?;

    Ok(customer)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_customer(
    pool: &PgPool,
    id: Uuid,
    req: &UpdateCustomerRequest,
) -> Result<Customer, AppError> {
    if let Some(ref email) = req.email {
        if !super::helpers::validate_email(email) {
            return Err(AppError::BadRequest(format!("invalid email: {email}")));
        }
    }

    let customer = sqlx::query_as::<_, Customer>(
        r#"
        UPDATE customers
        SET email = COALESCE($2, email),
            name = COALESCE($3, name),
            external_id = COALESCE($4, external_id),
            is_verified = COALESCE($5, is_verified),
            birthday = COALESCE($6, birthday),
            updated_at = now()
        WHERE id = $1
        RETURNING id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&req.email)
    .bind(&req.name)
    .bind(&req.external_id)
    .bind(req.is_verified)
    .bind(req.birthday)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("customer {id} not found")))?;

    Ok(customer)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn resolve_or_create(
    pool: &PgPool,
    req: &ResolveIdentityRequest,
) -> Result<(Customer, bool), AppError> {
    let normalized_phone = normalize_phone(&req.phone)?;

    let existing = sqlx::query_as::<_, Customer>(
        r#"
        SELECT id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
        FROM customers
        WHERE phone = $1
        "#,
    )
    .bind(&normalized_phone)
    .fetch_optional(pool)
    .await?;

    if let Some(customer) = existing {
        return Ok((customer, false));
    }

    if let Some(ref email) = req.email {
        if !super::helpers::validate_email(email) {
            return Err(AppError::BadRequest(format!("invalid email: {email}")));
        }
    }

    let customer = sqlx::query_as::<_, Customer>(
        r#"
        INSERT INTO customers (phone, email, name, external_id)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (phone) DO NOTHING
        RETURNING id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
        "#,
    )
    .bind(&normalized_phone)
    .bind(&req.email)
    .bind(&req.name)
    .bind(&req.external_id)
    .fetch_optional(pool)
    .await?;

    if let Some(customer) = customer {
        return Ok((customer, true));
    }

    let customer = sqlx::query_as::<_, Customer>(
        r#"
        SELECT id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
        FROM customers
        WHERE phone = $1
        "#,
    )
    .bind(&normalized_phone)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        AppError::Internal("customer upsert failed: row not found after insert".to_string())
    })?;

    Ok((customer, false))
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn search_customers(
    pool: &PgPool,
    query: &CustomerQuery,
) -> Result<Vec<Customer>, AppError> {
    match (&query.phone, &query.external_id) {
        (Some(phone), Some(external_id)) => {
            let normalized = normalize_phone(phone)?;
            let customers = sqlx::query_as::<_, Customer>(
                r#"
                SELECT id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
                FROM customers
                WHERE phone = $1 OR external_id = $2
                ORDER BY created_at DESC
                "#,
            )
            .bind(&normalized)
            .bind(external_id)
            .fetch_all(pool)
            .await?;

            Ok(customers)
        }
        (Some(phone), None) => {
            let normalized = normalize_phone(phone)?;
            let customers = sqlx::query_as::<_, Customer>(
                r#"
                SELECT id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
                FROM customers
                WHERE phone = $1
                ORDER BY created_at DESC
                "#,
            )
            .bind(&normalized)
            .fetch_all(pool)
            .await?;

            Ok(customers)
        }
        (None, Some(external_id)) => {
            let customers = sqlx::query_as::<_, Customer>(
                r#"
                SELECT id, phone, email, name, external_id, is_verified, birthday, created_at, updated_at
                FROM customers
                WHERE external_id = $1
                ORDER BY created_at DESC
                "#,
            )
            .bind(external_id)
            .fetch_all(pool)
            .await?;

            Ok(customers)
        }
        (None, None) => Err(AppError::BadRequest(
            "at least one of phone or external_id is required".to_string(),
        )),
    }
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_customers_with_birthday_today(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<CustomerWithWallet>, AppError> {
    let rows = sqlx::query_as::<_, CustomerWithWallet>(
        r#"
        SELECT c.id, c.phone, c.email, c.name, c.external_id, c.is_verified,
               c.birthday, c.created_at, c.updated_at, w.id AS wallet_id
        FROM wallets w
        JOIN customers c ON w.customer_id = c.id
        WHERE w.merchant_id = $1
          AND c.birthday IS NOT NULL
          AND EXTRACT(MONTH FROM c.birthday) = EXTRACT(MONTH FROM CURRENT_DATE)
          AND EXTRACT(DAY FROM c.birthday) = EXTRACT(DAY FROM CURRENT_DATE)
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
