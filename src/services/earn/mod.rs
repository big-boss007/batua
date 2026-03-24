mod handler;
pub mod helpers;
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
        .route("/earn/streaks", post(handler::create_streak_config))
        .route(
            "/earn/streaks/{merchant_id}",
            get(handler::list_streak_configs),
        )
        .route("/earn/check-streaks", post(handler::check_streaks))
        .route(
            "/earn/spin-wheel/config",
            post(handler::create_wheel_config),
        )
        .route("/earn/spin-wheel/spin", post(handler::spin_wheel))
        .route(
            "/earn/spin-wheel/{merchant_id}",
            get(handler::get_wheel_config),
        )
        .route(
            "/earn/memberships/assign",
            post(handler::assign_membership),
        )
        .route(
            "/earn/memberships/cancel/{membership_id}",
            post(handler::cancel_membership),
        )
        .route(
            "/earn/memberships/status/{merchant_id}/{customer_id}",
            get(handler::membership_status),
        )
        .route(
            "/earn/memberships/subscribers/{merchant_id}/enriched",
            get(handler::list_subscribers_enriched),
        )
        .route(
            "/earn/memberships/subscribers/{merchant_id}",
            get(handler::list_subscribers),
        )
        .route(
            "/earn/memberships/upgrade/{membership_id}",
            post(handler::upgrade_membership),
        )
        .route(
            "/earn/memberships/extend/{membership_id}",
            post(handler::extend_membership),
        )
        .route(
            "/earn/memberships/renew/{membership_id}",
            post(handler::renew_membership),
        )
}
