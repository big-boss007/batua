use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;
use crate::services::wallets::storage as wallet_storage;

use super::helpers;
use super::storage;
use super::types::{EligibilityQuery, InitiateRedemptionRequest, OrderContext};

#[tracing::instrument(skip(app_state))]
pub async fn initiate_redemption(
    State(app_state): State<AppState>,
    Json(req): Json<InitiateRedemptionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let wallet = wallet_storage::get_wallet(&app_state.db, req.wallet_id).await?;

    let redemption = storage::create_redemption(
        &app_state.db,
        wallet.merchant_id,
        req.wallet_id,
        &req.order_id,
        req.order_amount,
        req.payment_method.as_deref(),
        req.requested_amount,
    )
    .await?;

    let response = helpers::execute_redemption(&app_state.db, redemption.id).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_redemption(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let redemption = storage::get_redemption(pool, id).await?;

    Ok(Json(redemption))
}

#[tracing::instrument(skip(app_state))]
pub async fn compensate_redemption(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    helpers::compensate(&app_state.db, id).await?;
    let redemption = storage::get_redemption(&app_state.db, id).await?;

    Ok(Json(redemption))
}

#[tracing::instrument(skip(app_state))]
pub async fn check_eligibility(
    State(app_state): State<AppState>,
    Path(wallet_id): Path<Uuid>,
    Query(query): Query<EligibilityQuery>,
) -> Result<impl IntoResponse, AppError> {
    let order_context = OrderContext {
        order_id: String::new(),
        order_amount: query.order_amount,
        payment_method: query.payment_method,
        discount_codes: Vec::new(),
    };

    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let eligibility =
        helpers::evaluate_eligibility(pool, wallet_id, &order_context).await?;

    Ok(Json(eligibility))
}
