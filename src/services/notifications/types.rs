use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct NotificationTemplate {
    pub id: Uuid,
    pub merchant_id: Option<Uuid>,
    pub name: String,
    pub channel: String,
    pub locale: String,
    pub template_id: Option<String>,
    pub body_template: String,
    pub variables: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct NotificationLog {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    pub template_id: Uuid,
    pub channel: String,
    pub variables: serde_json::Value,
    pub status: String,
    pub external_message_id: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Connector {
    pub id: Uuid,
    pub merchant_id: Option<Uuid>,
    pub capability: String,
    pub vendor: String,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub merchant_id: Uuid,
    pub customer_id: Uuid,
    pub template_name: String,
    pub variables: serde_json::Value,
    pub channel_hint: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NotificationResult {
    pub log_id: Uuid,
    pub channel: String,
    pub status: String,
    pub external_message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub merchant_id: Option<Uuid>,
    pub name: String,
    pub channel: String,
    pub locale: Option<String>,
    pub template_id: Option<String>,
    pub body_template: String,
    pub variables: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub template_id: Option<String>,
    pub body_template: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateConnectorRequest {
    pub merchant_id: Option<Uuid>,
    pub capability: String,
    pub vendor: String,
    pub config: Option<serde_json::Value>,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConnectorRequest {
    pub config: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationTrigger {
    EarnCredit,
    RedeemCredit,
    CodPending,
    CodReleased,
    CodCancelled,
    Expiry,
    GiftCardReceived,
    ReferralReward,
    TierUpgrade,
}

impl NotificationTrigger {
    pub fn template_name(&self) -> &'static str {
        match self {
            Self::EarnCredit => "earn_credit",
            Self::RedeemCredit => "redeem_credit",
            Self::CodPending => "cod_pending",
            Self::CodReleased => "cod_released",
            Self::CodCancelled => "cod_cancelled",
            Self::Expiry => "expiry",
            Self::GiftCardReceived => "gift_card_received",
            Self::ReferralReward => "referral_reward",
            Self::TierUpgrade => "tier_upgrade",
        }
    }

    pub fn channel_capability(&self) -> &'static str {
        match self {
            Self::EarnCredit
            | Self::RedeemCredit
            | Self::CodPending
            | Self::CodReleased
            | Self::CodCancelled
            | Self::Expiry
            | Self::GiftCardReceived
            | Self::ReferralReward
            | Self::TierUpgrade => "whatsapp-bsp",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MerchantIdQuery {
    pub merchant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct NotificationLogQuery {
    pub merchant_id: Uuid,
    pub customer_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct NotificationLogPaginatedQuery {
    pub customer_id: Option<Uuid>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}
