mod handler;
mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post};

use crate::app_state::AppState;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/earn/process", post(handler::process_earn))
        .route("/earn/manual-credit", post(handler::manual_credit))
        .route("/earn/birthday-bonus", post(handler::birthday_bonus))
        .route("/earn/milestones", post(handler::create_milestone))
        .route("/earn/milestones/{merchant_id}", get(handler::list_milestones))
        .route("/earn/check-milestones", post(handler::check_milestones))
        .route(
            "/earn/milestones/{merchant_id}/{customer_id}",
            get(handler::get_customer_milestones),
        )
        .route(
            "/earn/newsletter-signup",
            post(handler::newsletter_signup),
        )
        .route(
            "/earn/newsletter-signups/{merchant_id}",
            get(handler::get_newsletter_signup_count),
        )
        .route(
            "/earn/profile-completion",
            post(handler::profile_completion),
        )
}
