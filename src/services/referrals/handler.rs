use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;
use crate::services::identity;

use super::helpers;
use super::storage;
use super::types::{CreateCodeRequest, CreateProgramRequest, PaginationQuery, ProcessReferralRequest};

#[tracing::instrument(skip(app_state))]
pub async fn create_program(
    State(app_state): State<AppState>,
    Json(req): Json<CreateProgramRequest>,
) -> impl IntoResponse {
    let program = storage::create_program(&app_state.db, &req).await?;
    Ok::<_, AppError>((StatusCode::CREATED, Json(program)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_program(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let program = storage::get_program(&app_state.db, merchant_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "no referral program found for merchant {merchant_id}"
            ))
        })?;

    Ok::<_, AppError>(Json(program))
}

#[tracing::instrument(skip(app_state))]
pub async fn create_code(
    State(app_state): State<AppState>,
    Json(req): Json<CreateCodeRequest>,
) -> impl IntoResponse {
    let code = if let Some(ref vanity) = req.code {
        vanity.to_uppercase()
    } else if req.is_vanity {
        let customer = identity::storage::get_customer(&app_state.db, req.customer_id).await?;
        helpers::generate_referral_code(customer.name.as_deref())
    } else {
        helpers::generate_referral_code(None)
    };

    let referral_code = storage::create_referral_code(&app_state.db, &req, &code).await?;
    Ok::<_, AppError>((StatusCode::CREATED, Json(referral_code)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_code(
    State(app_state): State<AppState>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let referral_code = storage::get_referral_code(&app_state.db, &code).await?;
    Ok::<_, AppError>(Json(referral_code))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_customer_code(
    State(app_state): State<AppState>,
    Path((merchant_id, customer_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let code = storage::get_customer_referral_code(pool, merchant_id, customer_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "no referral code found for customer {customer_id} in merchant {merchant_id}"
            ))
        })?;

    Ok::<_, AppError>(Json(code))
}

#[tracing::instrument(skip(app_state))]
pub async fn convert_referral(
    State(app_state): State<AppState>,
    Json(req): Json<ProcessReferralRequest>,
) -> impl IntoResponse {
    let result = helpers::process_referral(&app_state.db, &req).await?;
    Ok::<_, AppError>((StatusCode::OK, Json(result)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_analytics(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let analytics = storage::get_referral_analytics(&app_state.db, merchant_id).await?;
    Ok::<_, AppError>(Json(analytics))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_merchant_codes(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
    Query(params): Query<PaginationQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let codes = storage::list_merchant_referral_codes(pool, merchant_id, page, limit).await?;
    Ok::<_, AppError>(Json(codes))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_conversions(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
    Query(params): Query<PaginationQuery>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let conversions = storage::get_conversions(&app_state.db, merchant_id, page, limit).await?;
    Ok::<_, AppError>(Json(conversions))
}
