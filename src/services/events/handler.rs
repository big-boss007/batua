use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers::parse_shopify_webhook;
use super::storage;
use super::types::{
    EventResponse, IngestEventRequest, ListEventsQuery, ShopifyWebhookRequest,
};

#[tracing::instrument(skip(app_state))]
pub async fn ingest_event(
    State(app_state): State<AppState>,
    Json(req): Json<IngestEventRequest>,
) -> impl IntoResponse {
    let (event, is_duplicate) = storage::store_event(&app_state.db, &req).await?;

    let response = EventResponse {
        event_id: event.id,
        state: event.state,
        is_duplicate,
    };

    if is_duplicate {
        Ok::<_, AppError>(Json(response))
    } else {
        Ok(Json(response))
    }
}

#[tracing::instrument(skip(app_state))]
pub async fn shopify_order_webhook(
    State(app_state): State<AppState>,
    Json(req): Json<ShopifyWebhookRequest>,
) -> impl IntoResponse {
    let order = parse_shopify_webhook(&req.payload)?;

    let ingest_req = IngestEventRequest {
        merchant_id: req.merchant_id,
        event_type: "order.completed".to_string(),
        event_source: "shopify".to_string(),
        external_event_id: order.id.to_string(),
        payload: req.payload,
    };

    let (event, is_duplicate) = storage::store_event(&app_state.db, &ingest_req).await?;

    let response = EventResponse {
        event_id: event.id,
        state: event.state,
        is_duplicate,
    };

    Ok::<_, AppError>(Json(response))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_event(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let event = storage::get_event(pool, id).await?;
    Ok::<_, AppError>(Json(event))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_events(
    State(app_state): State<AppState>,
    Query(query): Query<ListEventsQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let events = storage::list_events(
        pool,
        query.merchant_id,
        query.event_type.as_deref(),
        query.state,
        limit,
        offset,
    )
    .await?;

    Ok::<_, AppError>(Json(events))
}
