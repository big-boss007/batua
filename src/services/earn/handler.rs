use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::storage;
use super::types::{
    CheckMilestonesRequest, CreateMilestoneRequest, ManualCreditRequest,
    NewsletterSignupRequest, ProcessBirthdayBonusRequest, ProcessEarnRequest,
};

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

#[tracing::instrument(skip(app_state))]
pub async fn create_milestone(
    State(app_state): State<AppState>,
    Json(req): Json<CreateMilestoneRequest>,
) -> impl IntoResponse {
    let config = storage::create_milestone_config(&app_state.db, &req).await?;
    Ok::<_, AppError>((StatusCode::CREATED, Json(config)))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_milestones(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let milestones = storage::get_active_milestones(&app_state.db, merchant_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(milestones)))
}

#[tracing::instrument(skip(app_state))]
pub async fn check_milestones(
    State(app_state): State<AppState>,
    Json(req): Json<CheckMilestonesRequest>,
) -> impl IntoResponse {
    let result =
        helpers::check_and_award_milestones(&app_state.db, req.merchant_id, req.customer_id)
            .await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_customer_milestones(
    State(app_state): State<AppState>,
    Path((merchant_id, customer_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let milestones =
        storage::get_customer_milestones(&app_state.db, merchant_id, customer_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(milestones)))
}

#[tracing::instrument(skip(app_state))]
pub async fn newsletter_signup(
    State(app_state): State<AppState>,
    Json(req): Json<NewsletterSignupRequest>,
) -> impl IntoResponse {
    let result = helpers::process_newsletter_signup(&app_state.db, req).await?;
    let status = if result.already_subscribed {
        StatusCode::OK
    } else {
        StatusCode::CREATED
    };
    Ok::<_, AppError>((status, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_newsletter_signup_count(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let count = storage::get_newsletter_signup_count(&app_state.db, merchant_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(count)))
}
