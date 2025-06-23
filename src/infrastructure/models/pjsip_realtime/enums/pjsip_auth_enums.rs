use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "auth_type", rename_all = "lowercase")]
pub enum AuthType {
    Userpass,
    Md5,
    GoogleOauth,
}

impl fmt::Display for AuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthType::Userpass => write!(f, "userpass"),
            AuthType::Md5 => write!(f, "md5"),
            AuthType::GoogleOauth => write!(f, "google_oauth"),
        }
    }
}
