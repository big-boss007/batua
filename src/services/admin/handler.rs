use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::storage;
use super::types::{
    BulkCreditRequest, CreateGeoPolicyRequest, CreateMerchantRequest, DisputeRequest,
    MerchantCustomersQuery, MerchantTransactionsQuery, PaginationQuery, RecentEventsQuery,
    UpdateMerchantRequest, UpdatePlanRequest, WalletPolicyRequest,
};

#[tracing::instrument(skip(app_state))]
pub async fn create_merchant(
    State(app_state): State<AppState>,
    Json(req): Json<CreateMerchantRequest>,
) -> impl IntoResponse {
    let merchant = storage::create_merchant(&app_state.db, &req).await?;

    Ok::<_, AppError>((axum::http::StatusCode::CREATED, Json(merchant)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_merchant(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let merchant = storage::get_merchant(pool, id).await?;

    Ok::<_, AppError>(Json(merchant))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_merchants(
    State(app_state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let merchants = storage::list_merchants(pool, page, limit).await?;

    Ok::<_, AppError>(Json(merchants))
}

#[tracing::instrument(skip(app_state))]
pub async fn update_merchant(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMerchantRequest>,
) -> impl IntoResponse {
    let merchant = storage::update_merchant(&app_state.db, id, &req).await?;

    Ok::<_, AppError>(Json(merchant))
}

#[tracing::instrument(skip(app_state))]
pub async fn bulk_credit(
    State(app_state): State<AppState>,
    Json(req): Json<BulkCreditRequest>,
) -> impl IntoResponse {
    let result = helpers::process_bulk_credit(&app_state.db, &req).await?;

    Ok::<_, AppError>(Json(result))
}

#[tracing::instrument(skip(app_state))]
pub async fn process_dispute(
    State(app_state): State<AppState>,
    Json(req): Json<DisputeRequest>,
) -> impl IntoResponse {
    let result = helpers::process_dispute(&app_state.db, &req).await?;

    Ok::<_, AppError>(Json(result))
}

#[tracing::instrument(skip(app_state))]
pub async fn create_wallet_policy(
    State(app_state): State<AppState>,
    Json(req): Json<WalletPolicyRequest>,
) -> impl IntoResponse {
    storage::create_wallet_policy(&app_state.db, &req).await?;

    Ok::<_, AppError>((
        axum::http::StatusCode::CREATED,
        Json(serde_json::json!({ "status": "ok" })),
    ))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_wallet_policies(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let policies = storage::get_wallet_policies(pool, merchant_id).await?;

    Ok::<_, AppError>(Json(policies))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_geo_policy(
    State(app_state): State<AppState>,
    Path(geo_code): Path<String>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let policy = storage::get_geo_policy(pool, &geo_code)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("geo policy '{geo_code}' not found")))?;

    Ok::<_, AppError>(Json(policy))
}

#[tracing::instrument(skip(app_state))]
pub async fn create_geo_policy(
    State(app_state): State<AppState>,
    Json(req): Json<CreateGeoPolicyRequest>,
) -> impl IntoResponse {
    let policy = storage::create_geo_policy(&app_state.db, &req).await?;

    Ok::<_, AppError>((axum::http::StatusCode::CREATED, Json(policy)))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_merchant_by_slug(
    State(app_state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let merchant = storage::get_merchant_by_slug(pool, &slug).await?;

    Ok::<_, AppError>(Json(merchant))
}

#[tracing::instrument(skip(app_state))]
pub async fn dashboard(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let stats = helpers::get_system_dashboard(pool).await?;

    Ok::<_, AppError>(Json(stats))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_all_geo_policies(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let policies = storage::list_geo_policies(pool).await?;

    Ok::<_, AppError>(Json(policies))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_merchant_stats(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let stats = storage::get_merchant_stats(pool, id).await?;

    Ok::<_, AppError>(Json(stats))
}

#[tracing::instrument(skip(app_state))]
pub async fn system_health(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let health = storage::get_system_health(pool).await?;

    Ok::<_, AppError>(Json(health))
}

#[tracing::instrument(skip(app_state))]
pub async fn recent_events(
    State(app_state): State<AppState>,
    Query(params): Query<RecentEventsQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let limit = params.limit.unwrap_or(10).clamp(1, 100);
    let events = storage::get_recent_events(pool, limit).await?;

    Ok::<_, AppError>(Json(events))
}

#[tracing::instrument(skip(app_state))]
pub async fn update_plan(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePlanRequest>,
) -> impl IntoResponse {
    let merchant = storage::update_merchant_plan(&app_state.db, id, &req.plan_tier).await?;

    Ok::<_, AppError>(Json(merchant))
}

#[tracing::instrument(skip(app_state))]
pub async fn merchant_dashboard(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let dashboard = storage::get_merchant_dashboard(pool, merchant_id).await?;

    Ok::<_, AppError>(Json(dashboard))
}

#[tracing::instrument(skip(app_state))]
pub async fn merchant_customers(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
    Query(params): Query<MerchantCustomersQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let customers = storage::list_merchant_customers(
        pool,
        merchant_id,
        params.search.as_deref(),
        page,
        limit,
    )
    .await?;

    Ok::<_, AppError>(Json(customers))
}

#[tracing::instrument(skip(app_state))]
pub async fn merchant_analytics(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let analytics = storage::get_merchant_analytics(pool, merchant_id).await?;

    Ok::<_, AppError>(Json(analytics))
}

#[tracing::instrument(skip(app_state))]
pub async fn merchant_transactions(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
    Query(params): Query<MerchantTransactionsQuery>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let transactions = storage::list_merchant_transactions(
        pool,
        merchant_id,
        params.search.as_deref(),
        params.bucket_type.as_deref(),
        params.movement_type.as_deref(),
        page,
        limit,
    )
    .await?;

    Ok::<_, AppError>(Json(transactions))
}
