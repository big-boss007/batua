mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};

use crate::app_state::AppState;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/cod/webhook/delivery", post(handler::delivery_webhook))
        .route("/cod/incentive", post(handler::cod_to_prepaid))
        .route(
            "/cod/orders/{merchant_id}",
            get(handler::list_cod_orders),
        )
        .route(
            "/cod/analytics/{merchant_id}",
            get(handler::cod_analytics),
        )
}
