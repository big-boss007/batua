use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub db_reader: Option<PgPool>,
    pub redis: redis::Client,
}
