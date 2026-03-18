mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};
use axum::Router;

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/wallets", post(handler::create_wallet))
        .route("/wallets/{id}", get(handler::get_wallet))
        .route("/wallets/lookup", get(handler::lookup_wallet))
        .route("/wallets/get-or-create", post(handler::get_or_create_wallet))
        .route(
            "/merchants/{merchant_id}/wallets",
            get(handler::list_wallets_for_merchant),
        )
}
