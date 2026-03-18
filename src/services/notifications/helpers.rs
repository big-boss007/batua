use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

use super::storage;
use super::types::{NotificationResult, NotificationTrigger, SendNotificationRequest};

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn send_notification(
    pool: &PgPool,
    req: &SendNotificationRequest,
) -> Result<NotificationResult, AppError> {
    let channel = req.channel_hint.as_deref().unwrap_or("whatsapp");
    let locale = req.locale.as_deref().unwrap_or("en");

    let template = storage::get_template(pool, req.merchant_id, &req.template_name, channel, locale)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "no active template '{}' for channel '{}' locale '{}'",
                req.template_name, channel, locale
            ))
        })?;

    let capability = match channel {
        "whatsapp" => "whatsapp-bsp",
        "sms" => "sms",
        "email" => "email",
        other => other,
    };

    let connector = storage::get_connector(pool, req.merchant_id, capability)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "no active connector for capability '{capability}'"
            ))
        })?;

    let rendered_body = render_template(&template.body_template, &req.variables);

    let log = storage::create_notification_log(
        pool,
        req.merchant_id,
        req.customer_id,
        template.id,
        channel,
        &req.variables,
    )
    .await?;

    tracing::info!(
        log_id = %log.id,
        vendor = %connector.vendor,
        channel = %channel,
        rendered_body = %rendered_body,
        "sending notification via connector (stub)"
    );

    let external_message_id = format!("stub-{}", log.id);

    storage::update_notification_status(pool, log.id, "sent", Some(&external_message_id)).await?;

    Ok(NotificationResult {
        log_id: log.id,
        channel: channel.to_string(),
        status: "sent".to_string(),
        external_message_id: Some(external_message_id),
    })
}

#[tracing::instrument]
pub fn render_template(body: &str, variables: &serde_json::Value) -> String {
    let mut rendered = body.to_string();

    if let Some(obj) = variables.as_object() {
        for (key, value) in obj {
            let placeholder = format!("{{{{{key}}}}}");
            let replacement = match value {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            rendered = rendered.replace(&placeholder, &replacement);
        }
    }

    rendered
}

#[tracing::instrument(skip(pool), err(Debug))]
pub async fn trigger_notification(
    pool: &PgPool,
    trigger: NotificationTrigger,
    merchant_id: Uuid,
    customer_id: Uuid,
    variables: serde_json::Value,
) -> Result<Option<NotificationResult>, AppError> {
    let template_name = trigger.template_name();
    let channel = "whatsapp";
    let locale = "en";

    let template =
        storage::get_template(pool, merchant_id, template_name, channel, locale).await?;

    if template.is_none() {
        tracing::info!(
            trigger = ?trigger,
            template_name = %template_name,
            "no template configured for trigger, skipping notification"
        );
        return Ok(None);
    }

    let req = SendNotificationRequest {
        merchant_id,
        customer_id,
        template_name: template_name.to_string(),
        variables,
        channel_hint: Some(channel.to_string()),
        locale: Some(locale.to_string()),
    };

    let result = send_notification(pool, &req).await?;
    Ok(Some(result))
}
