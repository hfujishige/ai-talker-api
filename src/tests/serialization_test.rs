#[cfg(test)]
mod tests {
    use crate::infrastructure::models::pjsip_realtime::{
        account::PjsipRealtimeAccountWithId,
        enums::pjsip_endpoint_enums::{RtpTimeout, TransportType},
    };

    #[test]
    fn test_rtp_timeout_serialization() {
        let account = PjsipRealtimeAccountWithId {
            id: "test_id".to_string(),
            username: "test_user".to_string(),
            password: "test_pass".to_string(),
            transport: TransportType::Ws,
            context: "from-sipproxy".to_string(),
            from_domain: "test.com".to_string(),
            from_user: "test_user".to_string(),
            rtp_timeout: Some(RtpTimeout::ThreeHundred),
            rtp_timeout_hold: Some(RtpTimeout::SixHundred),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&account).expect("Failed to serialize");
        println!("Serialized JSON: {}", json);
        
        assert!(json.contains("\"rtp_timeout\":300"));
        assert!(json.contains("\"rtp_timeout_hold\":600"));
    }
}
