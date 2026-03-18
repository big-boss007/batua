mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};

use crate::app_state::AppState;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/events/ingest", post(handler::ingest_event))
        .route("/events/shopify/orders", post(handler::shopify_order_webhook))
        .route("/events/{id}", get(handler::get_event))
        .route("/events", get(handler::list_events))
}
