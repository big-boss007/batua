use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::storage;
use super::types::{
    CreateConnectorRequest, CreateTemplateRequest, MerchantIdQuery, NotificationLogPaginatedQuery,
    NotificationLogQuery, SendNotificationRequest, UpdateTemplateRequest,
};

#[tracing::instrument(skip(app_state))]
pub async fn send_notification(
    State(app_state): State<AppState>,
    Json(req): Json<SendNotificationRequest>,
) -> impl IntoResponse {
    let result = helpers::send_notification(&app_state.db, &req).await?;
    Ok::<_, AppError>(Json(result))
}

#[tracing::instrument(skip(app_state))]
pub async fn create_template(
    State(app_state): State<AppState>,
    Json(req): Json<CreateTemplateRequest>,
) -> impl IntoResponse {
    let template = storage::create_template(&app_state.db, &req).await?;
    Ok::<_, AppError>(Json(template))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_templates(
    State(app_state): State<AppState>,
    Query(params): Query<MerchantIdQuery>,
) -> impl IntoResponse {
    let templates = storage::list_templates(&app_state.db, params.merchant_id).await?;
    Ok::<_, AppError>(Json(templates))
}

#[tracing::instrument(skip(app_state))]
pub async fn update_template(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTemplateRequest>,
) -> impl IntoResponse {
    let template = storage::update_template(&app_state.db, id, &req).await?;
    Ok::<_, AppError>(Json(template))
}

#[tracing::instrument(skip(app_state))]
pub async fn create_connector(
    State(app_state): State<AppState>,
    Json(req): Json<CreateConnectorRequest>,
) -> impl IntoResponse {
    let connector = storage::create_connector(&app_state.db, &req).await?;
    Ok::<_, AppError>(Json(connector))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_connectors(
    State(app_state): State<AppState>,
    Query(params): Query<MerchantIdQuery>,
) -> impl IntoResponse {
    let connectors = storage::list_connectors(&app_state.db, params.merchant_id).await?;
    Ok::<_, AppError>(Json(connectors))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_merchant_notification_logs(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
    Query(params): Query<NotificationLogPaginatedQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let logs = storage::list_notification_logs_paginated(
        pool,
        merchant_id,
        params.customer_id,
        page,
        limit,
    )
    .await?;
    Ok::<_, AppError>(Json(logs))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_notification_logs(
    State(app_state): State<AppState>,
    Query(params): Query<NotificationLogQuery>,
) -> impl IntoResponse {
    let logs =
        storage::list_notification_logs(&app_state.db, params.merchant_id, params.customer_id)
            .await?;
    Ok::<_, AppError>(Json(logs))
}
