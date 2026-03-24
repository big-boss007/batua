mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};

use crate::app_state::AppState;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/redemptions", post(handler::initiate_redemption))
        .route("/redemptions/{id}", get(handler::get_redemption))
        .route(
            "/redemptions/{id}/compensate",
            post(handler::compensate_redemption),
        )
        .route(
            "/wallets/{wallet_id}/eligibility",
            get(handler::check_eligibility),
        )
}
