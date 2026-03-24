use axum::extract::{Json, Path, State};
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::storage;
use super::types::{CreateProgramRequest, CreateTierRequest, UpdateProgramRequest, UpdateTierRequest};

#[tracing::instrument(skip(app_state))]
pub async fn create_program(
    State(app_state): State<AppState>,
    Json(req): Json<CreateProgramRequest>,
) -> Result<impl IntoResponse, AppError> {
    let program = storage::create_program(&app_state.db, &req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(program)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_program(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let program = storage::get_program(pool, merchant_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "no loyalty program found for merchant {merchant_id}"
            ))
        })?;
    Ok(Json(program))
}

#[tracing::instrument(skip(app_state))]
pub async fn update_program(
    State(app_state): State<AppState>,
    Path(program_id): Path<Uuid>,
    Json(req): Json<UpdateProgramRequest>,
) -> Result<impl IntoResponse, AppError> {
    let program = storage::update_program(
        &app_state.db,
        program_id,
        &req.name,
        &req.evaluation_criteria,
        req.evaluation_period_days,
        req.is_active,
    )
    .await?;
    Ok(Json(program))
}

#[tracing::instrument(skip(app_state))]
pub async fn create_tier(
    State(app_state): State<AppState>,
    Json(req): Json<CreateTierRequest>,
) -> Result<impl IntoResponse, AppError> {
    let tier = storage::create_tier(&app_state.db, &req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(tier)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_tiers(
    State(app_state): State<AppState>,
    Path(program_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let tiers = storage::get_tiers(pool, program_id).await?;
    Ok(Json(tiers))
}

#[tracing::instrument(skip(app_state))]
pub async fn update_tier(
    State(app_state): State<AppState>,
    Path(tier_id): Path<Uuid>,
    Json(req): Json<UpdateTierRequest>,
) -> Result<impl IntoResponse, AppError> {
    let tier = storage::update_tier(&app_state.db, tier_id, &req).await?;
    Ok(Json(tier))
}

#[tracing::instrument(skip(app_state))]
pub async fn delete_tier(
    State(app_state): State<AppState>,
    Path(tier_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    storage::delete_tier(&app_state.db, tier_id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

#[tracing::instrument(skip(app_state))]
pub async fn get_customer_tier_info(
    State(app_state): State<AppState>,
    Path((merchant_id, customer_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let info = helpers::get_customer_tier_info(pool, merchant_id, customer_id)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "no tier found for customer {customer_id} under merchant {merchant_id}"
            ))
        })?;
    Ok(Json(info))
}

#[tracing::instrument(skip(app_state))]
pub async fn evaluate_tier(
    State(app_state): State<AppState>,
    Path((merchant_id, customer_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let result = helpers::evaluate_tier(&app_state.db, merchant_id, customer_id).await?;
    Ok(Json(result))
}

#[tracing::instrument(skip(app_state))]
pub async fn evaluate_all_tiers(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let wallets = sqlx::query_as::<_, (Uuid,)>(
        "SELECT DISTINCT customer_id FROM wallets WHERE merchant_id = $1 AND customer_id IS NOT NULL",
    )
    .bind(merchant_id)
    .fetch_all(&app_state.db)
    .await?;

    let mut evaluated = 0i32;
    for (customer_id,) in &wallets {
        let _ = helpers::evaluate_tier(&app_state.db, merchant_id, *customer_id).await;
        evaluated += 1;
    }

    Ok(Json(serde_json::json!({ "evaluated": evaluated })))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_tier_distribution(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let distribution = storage::get_tier_distribution(pool, merchant_id).await?;
    Ok(Json(distribution))
}
