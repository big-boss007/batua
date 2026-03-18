mod handler;
mod helpers;
pub mod storage;
pub mod types;

use axum::routing::{get, post, put};
use axum::Router;

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/merchants", post(handler::create_merchant))
        .route("/admin/merchants", get(handler::list_merchants))
        .route("/admin/merchants/{id}", get(handler::get_merchant))
        .route("/admin/merchants/{id}", put(handler::update_merchant))
        .route(
            "/admin/merchants/by-slug/{slug}",
            get(handler::get_merchant_by_slug),
        )
        .route("/admin/bulk-credit", post(handler::bulk_credit))
        .route("/admin/disputes", post(handler::process_dispute))
        .route(
            "/admin/wallet-policies",
            post(handler::create_wallet_policy),
        )
        .route(
            "/admin/wallet-policies/{merchant_id}",
            get(handler::list_wallet_policies),
        )
        .route(
            "/admin/geo-policies/{geo_code}",
            get(handler::get_geo_policy),
        )
        .route(
            "/admin/geo-policies",
            post(handler::create_geo_policy).get(handler::list_all_geo_policies),
        )
        .route("/admin/dashboard", get(handler::dashboard))
        .route(
            "/admin/merchants/{id}/stats",
            get(handler::get_merchant_stats),
        )
        .route("/admin/system/health", get(handler::system_health))
        .route("/admin/events/recent", get(handler::recent_events))
        .route("/admin/merchants/{id}/plan", put(handler::update_plan))
        .route(
            "/admin/merchants/{merchant_id}/dashboard",
            get(handler::merchant_dashboard),
        )
        .route(
            "/admin/merchants/{merchant_id}/customers",
            get(handler::merchant_customers),
        )
        .route(
            "/admin/merchants/{merchant_id}/transactions",
            get(handler::merchant_transactions),
        )
        .route(
            "/admin/merchants/{merchant_id}/analytics",
            get(handler::merchant_analytics),
        )
}
