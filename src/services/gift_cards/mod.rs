mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};
use axum::Router;

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/gift-cards/issue", post(handler::issue_gift_card))
        .route("/gift-cards/bulk-issue", post(handler::bulk_issue))
        .route("/gift-cards/claim", post(handler::claim_gift_card))
        .route("/gift-cards/redeem", post(handler::redeem_gift_card))
        .route("/gift-cards/{code}", get(handler::get_gift_card_by_code))
        .route(
            "/gift-cards/merchant/{merchant_id}",
            get(handler::list_gift_cards_for_merchant),
        )
        .route(
            "/gift-cards/merchant/{merchant_id}/stats",
            get(handler::get_gift_card_stats),
        )
}
