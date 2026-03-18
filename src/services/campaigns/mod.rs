mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};

use crate::app_state::AppState;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/campaigns/templates", get(handler::list_templates))
        .route(
            "/campaigns/from-template",
            post(handler::create_from_template),
        )
        .route("/campaigns/calendar", get(handler::calendar))
        .route("/campaigns/{id}/performance", get(handler::performance))
}
