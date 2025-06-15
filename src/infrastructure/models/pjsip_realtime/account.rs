use crate::infrastructure::models::pjsip_realtime::enums::pjsip_endpoint_enums::TransportType;
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PjsipDeleteAccount {
    pub account_id: String,
}
