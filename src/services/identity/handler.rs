use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::AppError;

use super::storage;
use super::types::{CustomerQuery, IdentityResolution, ResolveIdentityRequest, UpdateCustomerRequest};

#[tracing::instrument(skip(app_state))]
pub async fn resolve_identity(
    State(app_state): State<AppState>,
    Json(req): Json<ResolveIdentityRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (customer, is_new) = storage::resolve_or_create(&app_state.db, &req).await?;

    let resolution = IdentityResolution {
        customer_id: customer.id,
        is_verified: customer.is_verified,
        is_new,
    };

    if is_new {
        Ok((StatusCode::CREATED, Json(resolution)))
    } else {
        Ok((StatusCode::OK, Json(resolution)))
    }
}

#[tracing::instrument(skip(app_state))]
pub async fn get_customer(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let customer = storage::get_customer(pool, id).await?;

    Ok(Json(customer))
}

#[tracing::instrument(skip(app_state))]
pub async fn update_customer(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCustomerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let customer = storage::update_customer(&app_state.db, id, &req).await?;

    Ok(Json(customer))
}

#[tracing::instrument(skip(app_state))]
pub async fn search_customers(
    State(app_state): State<AppState>,
    Query(query): Query<CustomerQuery>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db);
    let customers = storage::search_customers(pool, &query).await?;

    Ok(Json(customers))
}
