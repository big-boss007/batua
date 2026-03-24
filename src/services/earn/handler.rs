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
    AssignMembershipRequest, CheckMilestonesRequest, CheckStreakRequest, CreateMilestoneRequest,
    CreateStreakConfigRequest, CreateWheelRequest, ExtendMembershipRequest, ManualCreditRequest,
    NewsletterSignupRequest, ProcessBirthdayBonusRequest, ProcessEarnRequest,
    ProfileCompletionRequest, SpinRequest, UpgradeMembershipRequest,
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

#[tracing::instrument(skip(app_state))]
pub async fn profile_completion(
    State(app_state): State<AppState>,
    Json(req): Json<ProfileCompletionRequest>,
) -> impl IntoResponse {
    let result = helpers::process_profile_completion(&app_state.db, req).await?;
    let status = if result.already_rewarded {
        StatusCode::OK
    } else if result.rewarded {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok::<_, AppError>((status, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn create_streak_config(
    State(app_state): State<AppState>,
    Json(req): Json<CreateStreakConfigRequest>,
) -> impl IntoResponse {
    let config = storage::create_streak_config(&app_state.db, &req).await?;
    Ok::<_, AppError>((StatusCode::CREATED, Json(config)))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_streak_configs(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let configs = storage::get_active_streak_configs(&app_state.db, merchant_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(configs)))
}

#[tracing::instrument(skip(app_state))]
pub async fn check_streaks(
    State(app_state): State<AppState>,
    Json(req): Json<CheckStreakRequest>,
) -> impl IntoResponse {
    let result =
        helpers::check_and_award_streaks(&app_state.db, req.merchant_id, req.customer_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn create_wheel_config(
    State(app_state): State<AppState>,
    Json(req): Json<CreateWheelRequest>,
) -> impl IntoResponse {
    let result = helpers::create_wheel(&app_state.db, req).await?;
    Ok::<_, AppError>((StatusCode::CREATED, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_wheel_config(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let config = storage::get_wheel_config(&app_state.db, merchant_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound("no spin wheel configured for this merchant".to_string())
        })?;
    let segments = storage::get_wheel_segments(&app_state.db, config.id).await?;
    let result = super::types::WheelWithSegments {
        config,
        segments,
    };
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn spin_wheel(
    State(app_state): State<AppState>,
    Json(req): Json<SpinRequest>,
) -> impl IntoResponse {
    let result = helpers::spin_wheel(&app_state.db, req).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn assign_membership(
    State(app_state): State<AppState>,
    Json(req): Json<AssignMembershipRequest>,
) -> impl IntoResponse {
    let result = helpers::assign_membership(&app_state.db, req).await?;
    let status = if result.is_new {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok::<_, AppError>((status, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn cancel_membership(
    State(app_state): State<AppState>,
    Path(membership_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = helpers::cancel_membership_by_id(&app_state.db, membership_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn membership_status(
    State(app_state): State<AppState>,
    Path((merchant_id, customer_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let result = helpers::get_membership_status(&app_state.db, merchant_id, customer_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_subscribers(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let memberships = storage::list_memberships_by_merchant(&app_state.db, merchant_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(memberships)))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_subscribers_enriched(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let memberships = storage::list_enriched_memberships(pool, merchant_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(memberships)))
}

#[tracing::instrument(skip(app_state))]
pub async fn upgrade_membership(
    State(app_state): State<AppState>,
    Path(membership_id): Path<Uuid>,
    Json(req): Json<UpgradeMembershipRequest>,
) -> impl IntoResponse {
    let result =
        helpers::upgrade_membership(&app_state.db, membership_id, req.tier_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn extend_membership(
    State(app_state): State<AppState>,
    Path(membership_id): Path<Uuid>,
    Json(req): Json<ExtendMembershipRequest>,
) -> impl IntoResponse {
    let result =
        helpers::extend_membership(&app_state.db, membership_id, req.days).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn renew_membership(
    State(app_state): State<AppState>,
    Path(membership_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = helpers::renew_membership(&app_state.db, membership_id).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}
