use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::types::{ManualCreditRequest, ProcessEarnRequest};

#[tracing::instrument(skip(app_state))]
pub async fn process_earn(
    State(app_state): State<AppState>,
    Json(req): Json<ProcessEarnRequest>,
) -> impl IntoResponse {
    let result = helpers::process_earn(&app_state.db, req.event_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn manual_credit(
    State(app_state): State<AppState>,
    Json(req): Json<ManualCreditRequest>,
) -> impl IntoResponse {
    let result = helpers::process_manual_credit(&app_state.db, req).await?;
    Ok::<_, AppError>((StatusCode::CREATED, Json(result)))
}
