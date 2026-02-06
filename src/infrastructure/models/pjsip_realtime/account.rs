use crate::infrastructure::models::pjsip_realtime::enums::pjsip_endpoint_enums::{RtpTimeout, TransportType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

// TODO realm
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PjsipRealtimeAccount {
    pub username: String,
    pub password: String,
    pub transport: TransportType,
    pub context: String,
    pub from_domain: String,
    pub from_user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp_timeout: Option<RtpTimeout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp_timeout_hold: Option<RtpTimeout>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PjsipRealtimeAccountWithExternalId {
    pub id: String,
    pub username: String,
    pub password: String,
    pub transport: TransportType,
    pub context: String,
    pub from_domain: String,
    pub from_user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp_timeout: Option<RtpTimeout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtp_timeout_hold: Option<RtpTimeout>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PjsipRealtimeAccountWithId {
    pub id: String,
    pub username: String,
    pub password: String,
    pub transport: TransportType,
    pub context: String,
    pub from_domain: String,
    pub from_user: String,
    pub rtp_timeout: Option<RtpTimeout>,
    pub rtp_timeout_hold: Option<RtpTimeout>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PjsipDeleteAccount {
    pub account_id: String,
}
