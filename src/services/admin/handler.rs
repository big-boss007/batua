use axum::extract::{Json, Path, Query, State};
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::helpers;
use super::storage;
use super::types::{
    AdminDebitRequest, AdminExpireRequest, BulkCreditRequest, CoalitionTransferRequest,
    CreateCoalitionRequest, CreateGeoPolicyRequest, CreateMerchantRequest, DisputeRequest,
    MerchantCustomersQuery, MerchantTransactionsQuery, PaginationQuery, RecentEventsQuery,
    UpdateCustomerRequest, UpdateMerchantRequest, UpdatePlanRequest, WalletPolicyRequest,
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
pub async fn update_customer(
    State(app_state): State<AppState>,
    Path((merchant_id, customer_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateCustomerRequest>,
) -> impl IntoResponse {
    let customer = storage::update_customer(
        &app_state.db,
        merchant_id,
        customer_id,
        req.name.as_deref(),
        req.email.as_deref(),
    )
    .await?;

    Ok::<_, AppError>(Json(customer))
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

#[tracing::instrument(skip(app_state))]
pub async fn create_coalition(
    State(app_state): State<AppState>,
    Json(req): Json<CreateCoalitionRequest>,
) -> impl IntoResponse {
    if req.merchant_ids.len() < 2 {
        return Err(AppError::BadRequest(
            "a coalition requires at least 2 merchants".to_string(),
        ));
    }

    let coalition = storage::create_coalition(&app_state.db, &req.name).await?;

    let mut members = Vec::with_capacity(req.merchant_ids.len());
    for merchant_id in &req.merchant_ids {
        let member =
            storage::add_coalition_member(&app_state.db, coalition.id, *merchant_id, 1.0).await?;
        members.push(member);
    }

    Ok::<_, AppError>((
        axum::http::StatusCode::CREATED,
        Json(serde_json::json!({
            "coalition": coalition,
            "members": members,
        })),
    ))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_merchant_coalitions(
    State(app_state): State<AppState>,
    Path(merchant_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let coalitions = storage::get_merchant_coalitions(pool, merchant_id).await?;

    Ok::<_, AppError>(Json(coalitions))
}

#[tracing::instrument(skip(app_state))]
pub async fn coalition_transfer(
    State(app_state): State<AppState>,
    Json(req): Json<CoalitionTransferRequest>,
) -> impl IntoResponse {
    if req.amount <= 0.0 {
        return Err(AppError::BadRequest(
            "transfer amount must be positive".to_string(),
        ));
    }

    let result = helpers::transfer_coalition_credits(&app_state.db, &req).await?;

    Ok::<_, AppError>(Json(result))
}

#[tracing::instrument(skip(app_state))]
pub async fn get_coalition_transfers(
    State(app_state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let transfers = storage::get_coalition_transfers_for_customer(pool, customer_id).await?;

    Ok::<_, AppError>(Json(transfers))
}

#[tracing::instrument(skip(app_state))]
pub async fn admin_debit(
    State(app_state): State<AppState>,
    Json(req): Json<AdminDebitRequest>,
) -> impl IntoResponse {
    if req.amount <= 0.0 {
        return Err(AppError::BadRequest(
            "amount must be greater than zero".to_string(),
        ));
    }

    let result = helpers::process_debit(&app_state.db, &req).await?;

    Ok::<_, AppError>(Json(result))
}

#[tracing::instrument(skip(app_state))]
pub async fn admin_force_expire(
    State(app_state): State<AppState>,
    Json(req): Json<AdminExpireRequest>,
) -> impl IntoResponse {
    if req.bucket_types.is_empty() {
        return Err(AppError::BadRequest(
            "bucket_types must not be empty".to_string(),
        ));
    }

    let result = helpers::process_force_expire(&app_state.db, &req).await?;

    Ok::<_, AppError>(Json(result))
}
