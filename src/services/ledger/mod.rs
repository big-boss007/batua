mod handler;
mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};

use crate::app_state::AppState;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/entries", post(handler::create_entry))
        .route("/entries/{entry_id}", get(handler::get_entry_detail))
        .route("/wallets/{wallet_id}/entries", get(handler::get_entries))
        .route("/wallets/{wallet_id}/balance", get(handler::get_balance))
        .route(
            "/wallets/{wallet_id}/balance/at",
            get(handler::get_balance_at),
        )
}
