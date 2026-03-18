use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::storage;
use super::types::{
    BulkIssueRequest, ClaimGiftCardRequest, IssueGiftCardRequest, PaginationQuery,
    RedeemGiftCardRequest,
};

#[tracing::instrument(skip(app_state))]
pub async fn issue_gift_card(
    State(app_state): State<AppState>,
    Json(req): Json<IssueGiftCardRequest>,
) -> impl IntoResponse {
    let response = helpers::issue_gift_card(&app_state.db, req).await?;

    Ok::<_, AppError>((axum::http::StatusCode::CREATED, Json(response)))
}

#[tracing::instrument(skip(app_state))]
pub async fn bulk_issue(
    State(app_state): State<AppState>,
    Json(req): Json<BulkIssueRequest>,
) -> impl IntoResponse {
    let response = helpers::bulk_issue(&app_state.db, req).await?;

    Ok::<_, AppError>((axum::http::StatusCode::CREATED, Json(response)))
}

#[tracing::instrument(skip(app_state))]
pub async fn claim_gift_card(
    State(app_state): State<AppState>,
    Json(req): Json<ClaimGiftCardRequest>,
) -> impl IntoResponse {
    let response = helpers::claim_gift_card(&app_state.db, req).await?;

    Ok::<_, AppError>(Json(response))
}

#[tracing::instrument(skip(app_state))]
pub async fn redeem_gift_card(
    State(app_state): State<AppState>,
    Json(req): Json<RedeemGiftCardRequest>,
) -> impl IntoResponse {
    let response = helpers::redeem_gift_card(&app_state.db, req).await?;

    Ok::<_, AppError>(Json(response))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_gift_card_by_code(
    State(app_state): State<AppState>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let gift_card = storage::get_gift_card_by_code(pool, &code).await?;

    Ok::<_, AppError>(Json(gift_card.to_response()))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_gift_card_stats(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let stats = storage::get_gift_card_stats(pool, merchant_id).await?;

    Ok::<_, AppError>(Json(stats))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_gift_cards_for_merchant(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
    Query(params): Query<PaginationQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let cards = storage::list_gift_cards(pool, merchant_id, page, limit).await?;
    let responses: Vec<_> = cards.iter().map(|c| c.to_response()).collect();

    Ok::<_, AppError>(Json(responses))
}
