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
use super::types::{
    CreateCampaignRequest, CreateRuleRequest, EvaluateRequest, ListCampaignsQuery, ListRulesQuery,
    UpdateRuleRequest,
};

#[tracing::instrument(skip(app_state))]
pub async fn create_rule(
    State(app_state): State<AppState>,
    Json(req): Json<CreateRuleRequest>,
) -> impl IntoResponse {
    let rule = storage::create_rule(&app_state.db, &req).await?;
    Ok::<_, AppError>(Json(rule))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_rule(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let rule = storage::get_rule(pool, id).await?;
    Ok::<_, AppError>(Json(rule))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_rules(
    State(app_state): State<AppState>,
    Query(query): Query<ListRulesQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let rule_type = query.rule_type.as_deref().unwrap_or("reward");
    let rules = storage::get_active_rules(pool, query.merchant_id, rule_type).await?;
    Ok::<_, AppError>(Json(rules))
}

#[tracing::instrument(skip(app_state))]
pub async fn update_rule(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateRuleRequest>,
) -> impl IntoResponse {
    let rule = storage::update_rule(&app_state.db, id, req.name.as_deref(), &req.config, req.is_active).await?;
    Ok::<_, AppError>(Json(rule))
}

#[tracing::instrument(skip(app_state))]
pub async fn evaluate(
    State(app_state): State<AppState>,
    Json(req): Json<EvaluateRequest>,
) -> impl IntoResponse {
    let results = helpers::evaluate_rules(&app_state.db, &req.context).await?;
    Ok::<_, AppError>(Json(results))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_rule_performance(
    State(app_state): State<AppState>,
    Path(rule_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let performance = storage::get_rule_performance(pool, rule_id).await?;
    Ok::<_, AppError>(Json(performance))
}

#[tracing::instrument(skip(app_state))]
pub async fn create_campaign(
    State(app_state): State<AppState>,
    Json(req): Json<CreateCampaignRequest>,
) -> impl IntoResponse {
    let campaign = storage::create_campaign(&app_state.db, &req).await?;
    Ok::<_, AppError>(Json(campaign))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_campaigns(
    State(app_state): State<AppState>,
    Query(query): Query<ListCampaignsQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let active_only = query.active_only.unwrap_or(false);
    let campaigns = storage::list_campaigns(pool, query.merchant_id, active_only).await?;
    Ok::<_, AppError>(Json(campaigns))
}
