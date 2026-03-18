mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post, put};
use axum::Router;

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/notifications/send", post(handler::send_notification))
        .route(
            "/notifications/templates",
            post(handler::create_template).get(handler::list_templates),
        )
        .route(
            "/notifications/templates/{id}",
            put(handler::update_template),
        )
        .route(
            "/notifications/connectors",
            post(handler::create_connector).get(handler::list_connectors),
        )
        .route("/notifications/logs", get(handler::list_notification_logs))
        .route(
            "/notifications/logs/{merchant_id}",
            get(handler::list_merchant_notification_logs),
        )
}
