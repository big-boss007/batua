use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::storage;
use super::types::{
    CreateWalletRequest, GetOrCreateRequest, PaginationQuery, WalletLookupQuery, WalletResponse,
};

#[tracing::instrument(skip(app_state))]
pub async fn create_wallet(
    State(app_state): State<AppState>,
    Json(req): Json<CreateWalletRequest>,
) -> impl IntoResponse {
    let wallet = storage::create_wallet(&app_state.db, &req).await?;

    Ok::<_, AppError>(Json(WalletResponse {
        wallet,
        balance: None,
    }))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_wallet(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let wallet = storage::get_wallet(&app_state.db, id).await?;

    Ok::<_, AppError>(Json(WalletResponse {
        wallet,
        balance: None,
    }))
}

#[tracing::instrument(skip(app_state))]
pub async fn lookup_wallet(
    State(app_state): State<AppState>,
    Query(params): Query<WalletLookupQuery>,
) -> impl IntoResponse {
    let wallet =
        storage::get_wallet_by_merchant_customer(&app_state.db, params.merchant_id, params.customer_id)
            .await?
            .ok_or_else(|| AppError::NotFound("wallet not found".to_string()))?;

    Ok::<_, AppError>(Json(WalletResponse {
        wallet,
        balance: None,
    }))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_or_create_wallet(
    State(app_state): State<AppState>,
    Json(req): Json<GetOrCreateRequest>,
) -> impl IntoResponse {
    let wallet =
        storage::get_or_create_wallet(&app_state.db, req.merchant_id, req.customer_id).await?;

    Ok::<_, AppError>(Json(WalletResponse {
        wallet,
        balance: None,
    }))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_wallets_for_merchant(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
    Query(params): Query<PaginationQuery>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let wallets =
        storage::list_wallets_for_merchant(&app_state.db, merchant_id, page, limit).await?;

    Ok::<_, AppError>(Json(wallets))
}
