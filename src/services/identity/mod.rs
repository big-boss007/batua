mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post, put};
use axum::Router;

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/identity/resolve", post(handler::resolve_identity))
        .route("/identity/customers/{id}", get(handler::get_customer))
        .route("/identity/customers/{id}", put(handler::update_customer))
        .route("/identity/customers", get(handler::search_customers))
}
