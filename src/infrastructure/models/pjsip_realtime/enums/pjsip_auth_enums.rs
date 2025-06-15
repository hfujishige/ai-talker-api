use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum AuthType {
    Md5,
    Userpass,
    GoogleOauth,
}

impl fmt::Display for AuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthType::Md5 => write!(f, "transport-udp"),
            AuthType::Userpass => write!(f, "transport-tcp"),
            AuthType::GoogleOauth => write!(f, "transport-tls"),
        }
    }
}