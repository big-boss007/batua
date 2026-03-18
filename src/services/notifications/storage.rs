use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::types::{
    Connector, CreateConnectorRequest, CreateTemplateRequest, NotificationLog,
    NotificationTemplate, UpdateTemplateRequest,
};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_template(
    pool: &PgPool,
    merchant_id: Uuid,
    name: &str,
    channel: &str,
    locale: &str,
) -> Result<Option<NotificationTemplate>, AppError> {
    let template = sqlx::query_as::<_, NotificationTemplate>(
        r#"
        SELECT id, merchant_id, name, channel, locale, template_id,
               body_template, variables, is_active, created_at, updated_at
        FROM notification_templates
        WHERE merchant_id = $1 AND name = $2 AND channel = $3 AND locale = $4 AND is_active = true
        "#,
    )
    .bind(merchant_id)
    .bind(name)
    .bind(channel)
    .bind(locale)
    .fetch_optional(pool)
    .await?;

    Ok(template)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_template(
    pool: &PgPool,
    req: &CreateTemplateRequest,
) -> Result<NotificationTemplate, AppError> {
    let locale = req.locale.as_deref().unwrap_or("en");
    let variables = req
        .variables
        .as_ref()
        .cloned()
        .unwrap_or(serde_json::Value::Array(vec![]));

    let template = sqlx::query_as::<_, NotificationTemplate>(
        r#"
        INSERT INTO notification_templates (merchant_id, name, channel, locale, template_id, body_template, variables)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, merchant_id, name, channel, locale, template_id,
                  body_template, variables, is_active, created_at, updated_at
        "#,
    )
    .bind(req.merchant_id)
    .bind(&req.name)
    .bind(&req.channel)
    .bind(locale)
    .bind(&req.template_id)
    .bind(&req.body_template)
    .bind(&variables)
    .fetch_one(pool)
    .await?;

    Ok(template)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_templates(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<NotificationTemplate>, AppError> {
    let templates = sqlx::query_as::<_, NotificationTemplate>(
        r#"
        SELECT id, merchant_id, name, channel, locale, template_id,
               body_template, variables, is_active, created_at, updated_at
        FROM notification_templates
        WHERE merchant_id = $1
        ORDER BY name, channel, locale
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(templates)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_template(
    pool: &PgPool,
    id: Uuid,
    req: &UpdateTemplateRequest,
) -> Result<NotificationTemplate, AppError> {
    let existing = sqlx::query_as::<_, NotificationTemplate>(
        r#"
        SELECT id, merchant_id, name, channel, locale, template_id,
               body_template, variables, is_active, created_at, updated_at
        FROM notification_templates
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("template {id} not found")))?;

    let template_id = req.template_id.as_deref().unwrap_or(
        existing.template_id.as_deref().unwrap_or_default(),
    );
    let body_template = req
        .body_template
        .as_deref()
        .unwrap_or(&existing.body_template);
    let variables = req.variables.as_ref().unwrap_or(&existing.variables);
    let is_active = req.is_active.unwrap_or(existing.is_active);

    let updated = sqlx::query_as::<_, NotificationTemplate>(
        r#"
        UPDATE notification_templates
        SET template_id = $2, body_template = $3, variables = $4, is_active = $5, updated_at = now()
        WHERE id = $1
        RETURNING id, merchant_id, name, channel, locale, template_id,
                  body_template, variables, is_active, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(template_id)
    .bind(body_template)
    .bind(variables)
    .bind(is_active)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn get_connector(
    pool: &PgPool,
    merchant_id: Uuid,
    capability: &str,
) -> Result<Option<Connector>, AppError> {
    let connector = sqlx::query_as::<_, Connector>(
        r#"
        SELECT id, merchant_id, capability, vendor, config, is_active, priority, created_at, updated_at
        FROM connectors
        WHERE merchant_id = $1 AND capability = $2 AND is_active = true
        ORDER BY priority DESC
        LIMIT 1
        "#,
    )
    .bind(merchant_id)
    .bind(capability)
    .fetch_optional(pool)
    .await?;

    if connector.is_some() {
        return Ok(connector);
    }

    let system_default = sqlx::query_as::<_, Connector>(
        r#"
        SELECT id, merchant_id, capability, vendor, config, is_active, priority, created_at, updated_at
        FROM connectors
        WHERE merchant_id IS NULL AND capability = $1 AND is_active = true
        ORDER BY priority DESC
        LIMIT 1
        "#,
    )
    .bind(capability)
    .fetch_optional(pool)
    .await?;

    Ok(system_default)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_connector(
    pool: &PgPool,
    req: &CreateConnectorRequest,
) -> Result<Connector, AppError> {
    let config = req
        .config
        .as_ref()
        .cloned()
        .unwrap_or(serde_json::json!({}));
    let priority = req.priority.unwrap_or(0);

    let connector = sqlx::query_as::<_, Connector>(
        r#"
        INSERT INTO connectors (merchant_id, capability, vendor, config, priority)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, merchant_id, capability, vendor, config, is_active, priority, created_at, updated_at
        "#,
    )
    .bind(req.merchant_id)
    .bind(&req.capability)
    .bind(&req.vendor)
    .bind(&config)
    .bind(priority)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!(
                "connector already exists for capability '{}' and vendor '{}'",
                req.capability, req.vendor
            ))
        }
        other => AppError::Database(other),
    })?;

    Ok(connector)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_connectors(
    pool: &PgPool,
    merchant_id: Uuid,
) -> Result<Vec<Connector>, AppError> {
    let connectors = sqlx::query_as::<_, Connector>(
        r#"
        SELECT id, merchant_id, capability, vendor, config, is_active, priority, created_at, updated_at
        FROM connectors
        WHERE merchant_id = $1 OR merchant_id IS NULL
        ORDER BY merchant_id NULLS LAST, capability, priority DESC
        "#,
    )
    .bind(merchant_id)
    .fetch_all(pool)
    .await?;

    Ok(connectors)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn create_notification_log(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Uuid,
    template_id: Uuid,
    channel: &str,
    variables: &serde_json::Value,
) -> Result<NotificationLog, AppError> {
    let log = sqlx::query_as::<_, NotificationLog>(
        r#"
        INSERT INTO notification_logs (merchant_id, customer_id, template_id, channel, variables, status)
        VALUES ($1, $2, $3, $4, $5, 'pending')
        RETURNING id, merchant_id, customer_id, template_id, channel, variables, status,
                  external_message_id, sent_at, created_at
        "#,
    )
    .bind(merchant_id)
    .bind(customer_id)
    .bind(template_id)
    .bind(channel)
    .bind(variables)
    .fetch_one(pool)
    .await?;

    Ok(log)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn update_notification_status(
    pool: &PgPool,
    log_id: Uuid,
    status: &str,
    external_message_id: Option<&str>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE notification_logs
        SET status = $2, external_message_id = $3, sent_at = CASE WHEN $2 = 'sent' THEN now() ELSE sent_at END
        WHERE id = $1
        "#,
    )
    .bind(log_id)
    .bind(status)
    .bind(external_message_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_notification_logs(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Option<Uuid>,
) -> Result<Vec<NotificationLog>, AppError> {
    let logs = if let Some(cid) = customer_id {
        sqlx::query_as::<_, NotificationLog>(
            r#"
            SELECT id, merchant_id, customer_id, template_id, channel, variables, status,
                   external_message_id, sent_at, created_at
            FROM notification_logs
            WHERE merchant_id = $1 AND customer_id = $2
            ORDER BY created_at DESC
            "#,
        )
        .bind(merchant_id)
        .bind(cid)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, NotificationLog>(
            r#"
            SELECT id, merchant_id, customer_id, template_id, channel, variables, status,
                   external_message_id, sent_at, created_at
            FROM notification_logs
            WHERE merchant_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(merchant_id)
        .fetch_all(pool)
        .await?
    };

    Ok(logs)
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn list_notification_logs_paginated(
    pool: &PgPool,
    merchant_id: Uuid,
    customer_id: Option<Uuid>,
    page: i32,
    limit: i32,
) -> Result<Vec<NotificationLog>, AppError> {
    let offset = (page - 1) * limit;

    let logs = if let Some(cid) = customer_id {
        sqlx::query_as::<_, NotificationLog>(
            r#"
            SELECT id, merchant_id, customer_id, template_id, channel, variables, status,
                   external_message_id, sent_at, created_at
            FROM notification_logs
            WHERE merchant_id = $1 AND customer_id = $2
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(merchant_id)
        .bind(cid)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, NotificationLog>(
            r#"
            SELECT id, merchant_id, customer_id, template_id, channel, variables, status,
                   external_message_id, sent_at, created_at
            FROM notification_logs
            WHERE merchant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(merchant_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?
    };

    Ok(logs)
}
