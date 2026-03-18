use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub phone: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub external_id: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct IdentityResolution {
    pub customer_id: Uuid,
    pub is_verified: bool,
    pub is_new: bool,
}

#[derive(Debug, Deserialize)]
pub struct ResolveIdentityRequest {
    pub phone: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub external_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCustomerRequest {
    pub email: Option<String>,
    pub name: Option<String>,
    pub external_id: Option<String>,
    pub is_verified: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CustomerQuery {
    pub phone: Option<String>,
    pub external_id: Option<String>,
}
