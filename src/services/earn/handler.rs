use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::types::{ManualCreditRequest, ProcessBirthdayBonusRequest, ProcessEarnRequest};

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

#[tracing::instrument(skip(app_state))]
pub async fn birthday_bonus(
    State(app_state): State<AppState>,
    Json(req): Json<ProcessBirthdayBonusRequest>,
) -> impl IntoResponse {
    let result =
        helpers::process_birthday_bonuses(&app_state.db, req.merchant_id, req.amount).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}
