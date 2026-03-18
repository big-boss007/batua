mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};

use crate::app_state::AppState;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/rules", post(handler::create_rule).get(handler::list_rules))
        .route(
            "/rules/{id}",
            get(handler::get_rule).put(handler::update_rule),
        )
        .route("/rules/evaluate", post(handler::evaluate))
        .route(
            "/rules/{id}/performance",
            get(handler::get_rule_performance),
        )
        .route(
            "/campaigns",
            post(handler::create_campaign).get(handler::list_campaigns),
        )
}
