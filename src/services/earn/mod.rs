mod handler;
mod helpers;
mod storage;
mod types;

use axum::routing::post;

use crate::app_state::AppState;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/earn/process", post(handler::process_earn))
        .route("/earn/manual-credit", post(handler::manual_credit))
        .route("/earn/birthday-bonus", post(handler::birthday_bonus))
}
