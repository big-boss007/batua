use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::storage;
use super::types::{
    CodOrderResponse, CodOrdersQuery, CodToPrepaidRequest, DeliveryWebhookPayload,
};

#[tracing::instrument(skip(app_state))]
pub async fn delivery_webhook(
    State(app_state): State<AppState>,
    Json(payload): Json<DeliveryWebhookPayload>,
) -> impl IntoResponse {
    let merchant_id = payload.merchant_id.ok_or_else(|| {
        AppError::BadRequest("merchant_id is required in webhook payload".to_string())
    })?;

    match payload.status.as_str() {
        "delivered" => {
            helpers::process_delivery(&app_state.db, &payload.order_id, merchant_id).await?;
        }
        "rto" | "cancelled" => {
            helpers::process_rto(&app_state.db, &payload.order_id, merchant_id).await?;
        }
        other => {
            return Err(AppError::BadRequest(format!(
                "unknown delivery status: {other}"
            )));
        }
    }

    Ok::<_, AppError>((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "processed",
            "order_id": payload.order_id
        })),
    ))
}

#[tracing::instrument(skip(app_state))]
pub async fn cod_to_prepaid(
    State(app_state): State<AppState>,
    Json(req): Json<CodToPrepaidRequest>,
) -> impl IntoResponse {
    let result = helpers::process_cod_to_prepaid(&app_state.db, req).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_cod_orders(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
    Query(query): Query<CodOrdersQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).clamp(1, 100);

    let orders =
        storage::get_pending_cod_orders(pool, merchant_id, query.state.as_deref(), page, limit)
            .await?;

    let responses: Vec<CodOrderResponse> = orders
        .into_iter()
        .map(|o| CodOrderResponse {
            id: o.id,
            order_id: o.order_id,
            state: o.state,
            pending_amount: 0.0,
            created_at: o.created_at,
        })
        .collect();

    Ok::<_, AppError>(Json(responses))
}

#[tracing::instrument(skip(app_state))]
pub async fn cod_analytics(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let analytics = storage::get_cod_analytics(pool, merchant_id).await?;
    Ok::<_, AppError>(Json(analytics))
}
