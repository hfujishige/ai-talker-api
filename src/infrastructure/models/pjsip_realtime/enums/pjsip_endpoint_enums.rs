use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

// PJSIP Transport Type
#[derive(Debug, Clone, PartialEq, Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "transport_type", rename_all = "lowercase")]
pub enum TransportType {
    Udp,
    Tcp,
    Tls,
    Ws,
    Wss,
}

impl fmt::Display for TransportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let transport_str = match self {
            TransportType::Udp => "udp",
            TransportType::Tcp => "tcp",
            TransportType::Tls => "tls",
            TransportType::Ws => "ws",
            TransportType::Wss => "wss",
        };
        write!(f, "{}", transport_str)
    }
}

impl<'de> Deserialize<'de> for TransportType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "udp" | "Udp" => Ok(TransportType::Udp),
            "tcp" | "Tcp" => Ok(TransportType::Tcp),
            "tls" | "Tls" => Ok(TransportType::Tls),
            "ws" | "Ws" => Ok(TransportType::Ws),
            "wss" | "Wss" => Ok(TransportType::Wss),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid transport type: {}",
                s
            ))),
        }
    }
}
// End of Our system define

// pjsip fixed defines
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "connect_method", rename_all = "lowercase")]
pub enum ConnectMethod {
    Invite,
    Reinvite,
    Update,
}

impl fmt::Display for ConnectMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectMethod::Invite => write!(f, "invite"),
            ConnectMethod::Reinvite => write!(f, "reinvite"),
            ConnectMethod::Update => write!(f, "update"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "direct_media_glare_mitigation", rename_all = "lowercase")]
pub enum DirectMediaGlareMitigation {
    None,
    Outgoing,
    Incoming,
}

impl fmt::Display for DirectMediaGlareMitigation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirectMediaGlareMitigation::None => write!(f, "none"),
            DirectMediaGlareMitigation::Outgoing => write!(f, "outgoing"),
            DirectMediaGlareMitigation::Incoming => write!(f, "incoming"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "dtmf_mode", rename_all = "lowercase")]
pub enum DtmfMode {
    Rfc4733,
    Inband,
    Info,
    Auto,
    #[sqlx(rename = "auto_info")]
    Autoinfo,
}

impl fmt::Display for DtmfMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DtmfMode::Rfc4733 => write!(f, "rfc4733"),
            DtmfMode::Inband => write!(f, "inband"),
            DtmfMode::Info => write!(f, "info"),
            DtmfMode::Auto => write!(f, "auto"),
            DtmfMode::Autoinfo => write!(f, "auto_info"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "timers", rename_all = "snake_case")]
pub enum Timers {
    Forced,
    No,
    Required,
    Yes,
}

impl fmt::Display for Timers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Timers::Forced => write!(f, "forced"),
            Timers::No => write!(f, "no"),
            Timers::Required => write!(f, "required"),
            Timers::Yes => write!(f, "yes"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "caller_id_privacy", rename_all = "snake_case")]
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

impl fmt::Display for CallerIDPrivacy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallerIDPrivacy::AllowedNotScreened => write!(f, "allowed_not_screened"),
            CallerIDPrivacy::AllowedPassedScreened => write!(f, "allowed_passed_screened"),
            CallerIDPrivacy::AllowedFailedScreened => write!(f, "allowed_failed_screened"),
            CallerIDPrivacy::Allowed => write!(f, "allowed"),
            CallerIDPrivacy::ProhibNotScreened => write!(f, "prohib_not_screened"),
            CallerIDPrivacy::ProhibPassedScreened => write!(f, "prohib_passed_screened"),
            CallerIDPrivacy::ProhibFailedScreened => write!(f, "prohib_failed_screened"),
            CallerIDPrivacy::Prohib => write!(f, "prohib"),
            CallerIDPrivacy::Unavailable => write!(f, "unavailable"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "hundred_rel", rename_all = "snake_case")]
pub enum HundredRel {
    No,
    Required,
    PeerSupported,
    Yes,
}

impl fmt::Display for HundredRel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HundredRel::No => write!(f, "no"),
            HundredRel::Required => write!(f, "required"),
            HundredRel::PeerSupported => write!(f, "peer_supported"),
            HundredRel::Yes => write!(f, "yes"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "media_encryption", rename_all = "snake_case")]
pub enum MediaEncryption {
    No,
    Sdes,
    Dtls,
}

impl fmt::Display for MediaEncryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaEncryption::No => write!(f, "no"),
            MediaEncryption::Sdes => write!(f, "sdes"),
            MediaEncryption::Dtls => write!(f, "dtls"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "t38_udptl_ec", rename_all = "snake_case")]
pub enum T38UdptlEc {
    None,
    Fec,
    Redundancy,
}

impl fmt::Display for T38UdptlEc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            T38UdptlEc::None => write!(f, "none"),
            T38UdptlEc::Fec => write!(f, "fec"),
            T38UdptlEc::Redundancy => write!(f, "redundancy"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum DtlsSetup {
    Active,
    Passive,
    Actpass,
}

impl fmt::Display for DtlsSetup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DtlsSetup::Active => write!(f, "active"),
            DtlsSetup::Passive => write!(f, "passive"),
            DtlsSetup::Actpass => write!(f, "actpass"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "dtls_fingerprint")]
pub enum DtlsFingerprint {
    #[sqlx(rename = "SHA-1")]
    Sha1,
    #[sqlx(rename = "SHA-256")]
    Sha256,
}

impl fmt::Display for DtlsFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DtlsFingerprint::Sha1 => write!(f, "SHA-1"),
            DtlsFingerprint::Sha256 => write!(f, "SHA-256"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "redirect_method", rename_all = "snake_case")]
pub enum RedirectMethod {
    User,
    UriCore,
    UriPjsip,
}

impl fmt::Display for RedirectMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RedirectMethod::User => write!(f, "user"),
            RedirectMethod::UriCore => write!(f, "uri_core"),
            RedirectMethod::UriPjsip => write!(f, "uri_pjsip"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "incoming_call_offer_pref", rename_all = "snake_case")]
pub enum IncomingCallOfferPref {
    Local,
    LocalFirst,
    Remote,
    RemoteFirst,
}

impl fmt::Display for IncomingCallOfferPref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IncomingCallOfferPref::Local => write!(f, "local"),
            IncomingCallOfferPref::LocalFirst => write!(f, "local_first"),
            IncomingCallOfferPref::Remote => write!(f, "remote"),
            IncomingCallOfferPref::RemoteFirst => write!(f, "remote_first"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "outgoing_call_offer_pref", rename_all = "snake_case")]
pub enum OutgoingCallOfferPref {
    Local,
    LocalMerge,
    LocalFirst,
    Remote,
    RemoteMerge,
    RemoteFirst,
}

impl fmt::Display for OutgoingCallOfferPref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutgoingCallOfferPref::Local => write!(f, "local"),
            OutgoingCallOfferPref::LocalMerge => write!(f, "local_merge"),
            OutgoingCallOfferPref::LocalFirst => write!(f, "local_first"),
            OutgoingCallOfferPref::Remote => write!(f, "remote"),
            OutgoingCallOfferPref::RemoteMerge => write!(f, "remote_merge"),
            OutgoingCallOfferPref::RemoteFirst => write!(f, "remote_first"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "security_negotiation", rename_all = "snake_case")]
pub enum SecurityNegotiation {
    No,
    Mediasec,
}

impl fmt::Display for SecurityNegotiation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityNegotiation::No => write!(f, "no"),
            SecurityNegotiation::Mediasec => write!(f, "mediasec"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum RtpTimeout {
    Zero = 0,
    Fifteen = 15,
    Thirty = 30,
    Sixty = 60,
    Ninety = 90,
    OneTwenty = 120,
    OneEighty = 180,
    ThreeHundred = 300,
    SixHundred = 600,
}

impl Serialize for RtpTimeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.as_i32())
    }
}

impl<'de> Deserialize<'de> for RtpTimeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(RtpTimeout::Zero),
            15 => Ok(RtpTimeout::Fifteen),
            30 => Ok(RtpTimeout::Thirty),
            60 => Ok(RtpTimeout::Sixty),
            90 => Ok(RtpTimeout::Ninety),
            120 => Ok(RtpTimeout::OneTwenty),
            180 => Ok(RtpTimeout::OneEighty),
            300 => Ok(RtpTimeout::ThreeHundred),
            600 => Ok(RtpTimeout::SixHundred),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid RTP timeout value: {}. Valid values are: 0, 15, 30, 60, 90, 120, 180, 300, 600",
                value
            ))),
        }
    }
}

impl RtpTimeout {
    /// Returns the integer value for database storage
    pub fn as_i32(&self) -> i32 {
        match self {
            RtpTimeout::Zero => 0,
            RtpTimeout::Fifteen => 15,
            RtpTimeout::Thirty => 30,
            RtpTimeout::Sixty => 60,
            RtpTimeout::Ninety => 90,
            RtpTimeout::OneTwenty => 120,
            RtpTimeout::OneEighty => 180,
            RtpTimeout::ThreeHundred => 300,
            RtpTimeout::SixHundred => 600,
        }
    }
}

impl fmt::Display for RtpTimeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_i32())
    }
}
// end of pjsip fixed defines
