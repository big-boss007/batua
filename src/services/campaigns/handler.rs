use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::storage;
use super::types::{CampaignCalendarQuery, CreateFromTemplateRequest};

#[tracing::instrument(skip(app_state))]
pub async fn list_templates(State(app_state): State<AppState>) -> impl IntoResponse {
    let _ = &app_state;
    let templates = helpers::get_festive_templates();
    Json(templates)
}

#[tracing::instrument(skip(app_state))]
pub async fn create_from_template(
    State(app_state): State<AppState>,
    Json(req): Json<CreateFromTemplateRequest>,
) -> impl IntoResponse {
    let campaign = helpers::create_from_template(&app_state.db, &req).await?;
    Ok::<_, AppError>(Json(campaign))
}

#[tracing::instrument(skip(app_state))]
pub async fn calendar(
    State(app_state): State<AppState>,
    Query(query): Query<CampaignCalendarQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let entries = helpers::get_calendar(pool, &query).await?;
    Ok::<_, AppError>(Json(entries))
}

#[tracing::instrument(skip(app_state))]
pub async fn performance(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let perf = storage::get_campaign_performance(pool, id).await?;
    Ok::<_, AppError>(Json(perf))
}
