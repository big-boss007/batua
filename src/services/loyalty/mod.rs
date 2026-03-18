mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};

use crate::app_state::AppState;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/loyalty/programs", post(handler::create_program))
        .route(
            "/loyalty/programs/{merchant_id}",
            get(handler::get_program),
        )
        .route("/loyalty/tiers", post(handler::create_tier))
        .route(
            "/loyalty/programs/{program_id}/tiers",
            get(handler::get_tiers),
        )
        .route(
            "/loyalty/customers/{merchant_id}/{customer_id}",
            get(handler::get_customer_tier_info),
        )
        .route(
            "/loyalty/evaluate/{merchant_id}/{customer_id}",
            post(handler::evaluate_tier),
        )
        .route(
            "/loyalty/distribution/{merchant_id}",
            get(handler::get_tier_distribution),
        )
}
