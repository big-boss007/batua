mod handler;
pub mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};
use axum::Router;

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/referrals/programs", post(handler::create_program))
        .route(
            "/referrals/programs/{merchant_id}",
            get(handler::get_program),
        )
        .route("/referrals/codes", post(handler::create_code))
        .route("/referrals/codes/{code}", get(handler::get_code))
        .route(
            "/referrals/codes/customer/{merchant_id}/{customer_id}",
            get(handler::get_customer_code),
        )
        .route("/referrals/convert", post(handler::convert_referral))
        .route(
            "/referrals/analytics/{merchant_id}",
            get(handler::get_analytics),
        )
        .route(
            "/referrals/merchant/{merchant_id}/codes",
            get(handler::list_merchant_codes),
        )
        .route(
            "/referrals/conversions/{merchant_id}",
            get(handler::list_conversions),
        )
}
