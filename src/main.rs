use std::env;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use batua::app_state::AppState;
use batua::helper::{propagate_request_id_layer, set_request_id_layer};
use batua::services;

fn get_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .merge(services::ledger::router().with_state(app_state.clone()))
        .merge(services::wallets::router().with_state(app_state.clone()))
        .merge(services::events::router().with_state(app_state.clone()))
        .merge(services::rules::router().with_state(app_state.clone()))
        .merge(services::identity::router().with_state(app_state.clone()))
        .merge(services::earn::router().with_state(app_state.clone()))
        .merge(services::redemption::router().with_state(app_state.clone()))
        .merge(services::cod::router().with_state(app_state.clone()))
        .merge(services::notifications::router().with_state(app_state.clone()))
        .merge(services::campaigns::router().with_state(app_state.clone()))
        .merge(services::loyalty::router().with_state(app_state.clone()))
        .merge(services::gift_cards::router().with_state(app_state.clone()))
        .merge(services::referrals::router().with_state(app_state.clone()))
        .merge(services::admin::router().with_state(app_state))
}

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let log_format = env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    match log_format.as_str() {
        "json" => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        }
        _ => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(tracing_subscriber::fmt::layer().pretty())
                .init();
        }
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    let db_reader = match env::var("DATABASE_READER_URL") {
        Ok(url) if !url.is_empty() => {
            let pool = PgPoolOptions::new()
                .max_connections(20)
                .connect(&url)
                .await
                .expect("failed to connect to reader database");
            Some(pool)
        }
        _ => None,
    };

    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let redis = redis::Client::open(redis_url).expect("failed to create redis client");

    let app_state = AppState {
        db,
        db_reader,
        redis,
    };

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let app = get_router(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(set_request_id_layer())
        .layer(propagate_request_id_layer());

    tracing::info!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
