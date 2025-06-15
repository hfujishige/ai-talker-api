use std::fmt;
use serde::{Deserialize, Serialize};

// Our system define
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "kebab-case")]
pub enum TransportType {
    TransportUdp,
    TransportTcp,
    TransportTls,
    TransportWs,
    TransportWss,
}

impl fmt::Display for TransportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransportType::TransportUdp => write!(f, "transport-udp"),
            TransportType::TransportTcp => write!(f, "transport-tcp"),
            TransportType::TransportTls => write!(f, "transport-tls"),
            TransportType::TransportWs => write!(f, "transport-ws"),
            TransportType::TransportWss => write!(f, "transport-wss"),
        }
    }
}
// End of Our system define

// pjsip fixed defines
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "lowercase")]
pub enum ConnectMethod {
    Invite,
    Reinvite,
    Update,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "lowercase")]
pub enum DirectMediaGlareMitigation {
    None,
    Outgoing,
    Incoming,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "lowercase")]
pub enum DtmfMode {
    Rfc4733,
    Inband,
    Info,
    Auto,
    #[sqlx(rename = "auto_info")] Autoinfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "lowercase")]
pub enum Timers {
    Forced,
    No,
    Required,
    Yes,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum CallerIDPrivacy {
    AllowedNotScreened,
    AllowedPassedScreened,
    AllowedFailedScreened,
    Allowed,
    ProhibNotScreened,
    ProhibPassedScreened,
    ProhibFailedScreened,
    Prohib,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum HundredRel {
    No,
    Required,
    PeerSupported,
    Yes,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum MediaEncryption {
    No,
    Sdes,
    Dtls,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum T38UdptlEc {
    None,
    Fec,
    Redundancy,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum DtlsSetup {
    Active,
    Passive,
    Actpass,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM")]
pub enum DtlsFingerprint {
    #[sqlx(rename = "SHA-1")] Sha1,
    #[sqlx(rename = "SHA-256")] Sha256,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum RedirectMethod {
    User,
    UriCore,
    UriPjsip,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum IncomingCallOfferPref {
    Local,
    LocalFirst,
    Remote,
    RemoteFirst,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum OutgoingCallOfferPref {
    Local,
    LocalMerge,
    LocalFirst,
    Remote,
    RemoteMerge,
    RemoteFirst,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum SecurityNegotiation {
    No,
    Mediasec,
}
// end of pjsip fixed defines
