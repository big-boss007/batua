use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers::generate_idempotency_key;
use super::storage;
use super::types::{
    BalanceAtQuery, CreateEntryRequest, GetEntriesQuery, NewLedgerEntry,
};

#[tracing::instrument(skip(app_state))]
pub async fn create_entry(
    State(app_state): State<AppState>,
    Json(req): Json<CreateEntryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let idempotency_key = match req.idempotency_key {
        Some(ref key) => key.clone(),
        None => generate_idempotency_key(
            &req.wallet_id.to_string(),
            req.event_id,
            req.rule_snapshot_id,
        ),
    };

    let new_entry = NewLedgerEntry {
        wallet_id: req.wallet_id,
        bucket_type: req.bucket_type,
        movement_type: req.movement_type,
        earning_unit: req.earning_unit,
        currency_equivalent: req.currency_equivalent,
        conversion_rate: req.conversion_rate,
        event_id: req.event_id,
        rule_snapshot_id: req.rule_snapshot_id,
        campaign_snapshot_id: req.campaign_snapshot_id,
        actor_type: req.actor_type,
        actor_id: req.actor_id,
        payment_reference: req.payment_reference,
        transfer_id: req.transfer_id,
        constraints: req.constraints.unwrap_or(serde_json::json!({})),
        expires_at: req.expires_at,
    };

    let entry = storage::create_entry(&app_state.db, new_entry, idempotency_key).await?;

    Ok((axum::http::StatusCode::CREATED, Json(entry)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_entries(
    State(app_state): State<AppState>,
    Path(wallet_id): Path<Uuid>,
    Query(query): Query<GetEntriesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let entries = storage::get_entries(pool, wallet_id, query).await?;

    Ok(Json(entries))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_balance(
    State(app_state): State<AppState>,
    Path(wallet_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let balance = storage::get_balance(pool, wallet_id).await?;

    Ok(Json(balance))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_balance_at(
    State(app_state): State<AppState>,
    Path(wallet_id): Path<Uuid>,
    Query(query): Query<BalanceAtQuery>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let balance = storage::get_balance_at(pool, wallet_id, query.at).await?;

    Ok(Json(balance))
}
