use crate::infrastructure::models::{
    errors::{deletion_error::DeletionError, registration_error::RegistrationError},
    pjsip_realtime::{
        account::PjsipRealtimeAccountWithId,
        enums::pjsip_endpoint_enums::TransportType,
        sip_udp::{PsAorForUdp, PsAuthForUdp, PsEndpointForUdp},
        sip_ws::{PsAorForWs, PsAuthForWs, PsEndpointForWs},
    },
};
use axum::http::StatusCode;
use sqlx::{PgPool, Postgres, Row, Transaction, postgres::PgQueryResult};

// registration method
pub async fn exec_insert_udp_pjsip_account(
    transaction: &mut Transaction<'_, Postgres>,
    account: &PjsipRealtimeAccountWithId,
    auth: &PsAuthForUdp,
    aor: &PsAorForUdp,
    endpoint: &PsEndpointForUdp,
) -> Result<StatusCode, RegistrationError> {
    // Validate the input data
    // 事前バリデーション
    if auth.id.is_empty() || aor.id.is_empty() || endpoint.id.is_empty() {
        return Err(RegistrationError::ValidationError(
            "ID cannot be empty".to_string(),
        ));
    }

    // check duplicate account
    // check exist record.
    let exists: bool = sqlx::query_scalar(
        r#"SELECT EXISTS(SELECT 1 FROM pjsip_realtime_accounts WHERE id = $1 or username = $2)"#,
    )
    .bind(&account.id)
    .bind(&account.username)
    .fetch_one(&mut **transaction)
    .await?;

    // Check if the record count is 0 (no records exist)
    println!("Checking account with ID {} exists: {}", account.id, exists);
    if exists {
        return Err(RegistrationError::DuplicateError);
    }

    // Insert SQL statements for pjsip_realtime tables with placeholders
    // NOTE: This requires the enum definitions to have the correct `#[sqlx(type_name = "...")]` attribute.
    let account_insert: &'static str = r#"
        INSERT INTO pjsip_realtime_accounts
        (id, username, password, transport, context, from_domain, from_user, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"#;
    let auth_insert: &'static str = r#"
        insert into ps_auths (id, auth_type, password, username)
        values ($1, $2::pjsip_auth_type_values_v2, $3, $4)"#;
    let aor_insert: &'static str = r#"
        insert into ps_aors (id, default_expiration, max_contacts, minimum_expiration,
                             qualify_frequency, maximum_expiration, qualify_timeout,
                             remove_existing, remove_unavailable)
        values ($1, $2, $3, $4, $5, $6, $7, $8::ast_bool_values, $9::ast_bool_values)"#;
    let endpoint_insert: &'static str = r#"
        insert into ps_endpoints (id, transport, auth, aors, context, disallow, allow, direct_media,
                                  force_rport, rewrite_contact, rtp_symmetric, media_encryption,
                                  from_domain, from_user, dtmf_mode)
        values ($1, $2::transport_type, $3, $4, $5, $6, $7, $8::ast_bool_values,
                $9::ast_bool_values, $10::ast_bool_values, $11::ast_bool_values, $12::pjsip_media_encryption_values,
                $13, $14, $15::pjsip_dtmf_mode_values_v3)"#;
    // TODO : define result types MySqlQueryResult to PgQueryResult after migrate mysql to postgres)

    let account_result: PgQueryResult = sqlx::query(account_insert)
        .bind(&account.id)
        .bind(&account.username)
        .bind(&account.password)
        .bind(account.transport.to_string())
        .bind(&account.context)
        .bind(&account.from_domain)
        .bind(&account.from_user)
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    let auth_result: PgQueryResult = sqlx::query(auth_insert)
        .bind(&auth.id)
        .bind(&auth.auth_type.to_string())
        .bind(&auth.password)
        .bind(&auth.username)
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    let aor_result: PgQueryResult = sqlx::query(aor_insert)
        .bind(&aor.id)
        .bind(&aor.default_expiration)
        .bind(&aor.max_contacts)
        .bind(&aor.minimum_expiration)
        .bind(&aor.qualify_frequency)
        .bind(&aor.maximum_expiration)
        .bind(&aor.qualify_timeout)
        .bind(&aor.remove_existing.to_string())
        .bind(&aor.remove_unavailable.to_string())
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    let endpoint_result: PgQueryResult = sqlx::query(endpoint_insert)
        .bind(&endpoint.id)
        .bind(&endpoint.transport.to_string())
        .bind(&endpoint.auth)
        .bind(&endpoint.aors)
        .bind(&endpoint.context)
        .bind(&endpoint.disallow)
        .bind(&endpoint.allow)
        .bind(&endpoint.direct_media.to_string())
        .bind(&endpoint.force_rport.to_string())
        .bind(&endpoint.rewrite_contact.to_string())
        .bind(&endpoint.rtp_symmetric.to_string())
        .bind(&endpoint.media_encryption.to_string())
        .bind(&endpoint.from_domain)
        .bind(&endpoint.from_user)
        .bind(&endpoint.dtmf_mode.to_string())
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    // if any of the insertions failed, return an error
    if account_result.rows_affected() == 0
        || auth_result.rows_affected() == 0
        || aor_result.rows_affected() == 0
        || endpoint_result.rows_affected() == 0
    {
        return Err(RegistrationError::InsertionFailed);
    }
    // return OK status with a success message
    Ok(StatusCode::CREATED)
}

// registration method for WebSocket transport
pub async fn exec_insert_ws_pjsip_account(
    transaction: &mut Transaction<'_, Postgres>,
    account: &PjsipRealtimeAccountWithId,
    auth: &PsAuthForWs,
    aor: &PsAorForWs,
    endpoint: &PsEndpointForWs,
) -> Result<StatusCode, RegistrationError> {
    // Validate the input data
    if auth.id.is_empty() || aor.id.is_empty() || endpoint.id.is_empty() {
        return Err(RegistrationError::ValidationError(
            "ID cannot be empty".to_string(),
        ));
    }

    // check duplicate account
    let exists: bool = sqlx::query_scalar(
        r#"SELECT EXISTS(SELECT 1 FROM pjsip_realtime_accounts WHERE id = $1 or username = $2)"#,
    )
    .bind(&account.id)
    .bind(&account.username)
    .fetch_one(&mut **transaction)
    .await?;

    println!("Checking account with ID {} exists: {}", account.id, exists);
    if exists {
        return Err(RegistrationError::DuplicateError);
    }

    // Insert SQL statements for pjsip_realtime tables
    let account_insert: &'static str = r#"
        INSERT INTO pjsip_realtime_accounts
        (id, username, password, transport, context, from_domain, from_user, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"#;
    let auth_insert: &'static str = r#"
        insert into ps_auths (id, auth_type, password, username)
        values ($1, $2::pjsip_auth_type_values_v2, $3, $4)"#;
    let aor_insert: &'static str = r#"
        insert into ps_aors (id, default_expiration, max_contacts, minimum_expiration,
                             maximum_expiration, remove_existing, remove_unavailable)
        values ($1, $2, $3, $4, $5, $6::ast_bool_values, $7::ast_bool_values)"#;
    let endpoint_insert: &'static str = r#"
        insert into ps_endpoints (id, transport, aors, auth, context, disallow, allow, direct_media,
                                  force_rport, rewrite_contact, rtp_symmetric, media_encryption,
                                  from_domain, from_user, dtmf_mode, rtp_ipv6, ice_support, use_avpf,
                                  webrtc, max_audio_streams, max_video_streams, rtp_timeout, rtp_timeout_hold)
        values ($1, $2::transport_type, $3, $4, $5, $6, $7, $8::ast_bool_values,
                $9::ast_bool_values, $10::ast_bool_values, $11::ast_bool_values, $12::pjsip_media_encryption_values,
                $13, $14, $15::pjsip_dtmf_mode_values_v3, $16::ast_bool_values, $17::ast_bool_values, $18::ast_bool_values,
                $19::ast_bool_values, $20, $21, $22, $23)"#;

    let account_result: PgQueryResult = sqlx::query(account_insert)
        .bind(&account.id)
        .bind(&account.username)
        .bind(&account.password)
        .bind(account.transport.to_string())
        .bind(&account.context)
        .bind(&account.from_domain)
        .bind(&account.from_user)
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    let auth_result: PgQueryResult = sqlx::query(auth_insert)
        .bind(&auth.id)
        .bind(&auth.auth_type.to_string())
        .bind(&auth.password)
        .bind(&auth.username)
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    let aor_result: PgQueryResult = sqlx::query(aor_insert)
        .bind(&aor.id)
        .bind(&aor.default_expiration)
        .bind(&aor.max_contacts)
        .bind(&aor.minimum_expiration)
        .bind(&aor.maximum_expiration)
        .bind(&aor.remove_existing.to_string())
        .bind(&aor.remove_unavailable.to_string())
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    let endpoint_result: PgQueryResult = sqlx::query(endpoint_insert)
        .bind(&endpoint.id)
        .bind(&endpoint.transport.to_string())
        .bind(&endpoint.aors)
        .bind(&endpoint.auth)
        .bind(&endpoint.context)
        .bind(&endpoint.disallow)
        .bind(&endpoint.allow)
        .bind(&endpoint.direct_media.to_string())
        .bind(&endpoint.force_rport.to_string())
        .bind(&endpoint.rewrite_contact.to_string())
        .bind(&endpoint.rtp_symmetric.to_string())
        .bind(&endpoint.media_encryption.to_string())
        .bind(&endpoint.from_domain)
        .bind(&endpoint.from_user)
        .bind(&endpoint.dtmf_mode.to_string())
        .bind(&endpoint.rtp_ipv6.to_string())
        .bind(endpoint.ice_support.as_ref().map(|v| v.to_string()))
        .bind(endpoint.use_avpf.as_ref().map(|v| v.to_string()))
        .bind(endpoint.webrtc.as_ref().map(|v| v.to_string()))
        .bind(&endpoint.max_audio_streams)
        .bind(&endpoint.max_video_streams)
        .bind(endpoint.rtp_timeout.as_ref().map(|v| v.as_i32()))
        .bind(endpoint.rtp_timeout_hold.as_ref().map(|v| v.as_i32()))
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;

    // if any of the insertions failed, return an error
    if account_result.rows_affected() == 0
        || auth_result.rows_affected() == 0
        || aor_result.rows_affected() == 0
        || endpoint_result.rows_affected() == 0
    {
        return Err(RegistrationError::InsertionFailed);
    }
    // return OK status with a success message
    Ok(StatusCode::CREATED)
}

pub async fn exec_delete_pjsip_account(
    transaction: &mut Transaction<'_, Postgres>,
    account_id: String,
) -> Result<StatusCode, DeletionError> {
    // Validate the account_id
    if account_id.is_empty() {
        return Err(DeletionError::IdNotSpecified);
    }

    // check exist record.
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pjsip_realtime_accounts WHERE id = $1)")
            .bind(&account_id)
            .fetch_one(&mut **transaction)
            .await?;

    // Check if the record count is 0 (no records exist)
    if !exists {
        return Err(DeletionError::NotFoundRecord);
    }

    // Delete SQL statements for pjsip_realtime tables with placeholders
    // NOTE: This requires the enum definitions to have the correct `#[sqlx(type_name = "...")]` attribute.
    sqlx::query("DELETE FROM pjsip_realtime_accounts WHERE id = $1")
        .bind(&account_id)
        .execute(&mut **transaction)
        .await?;

    // テーブル名を複数形に修正
    sqlx::query("DELETE FROM ps_endpoints WHERE id = $1")
        .bind(&account_id)
        .execute(&mut **transaction)
        .await?;

    sqlx::query("DELETE FROM ps_aors WHERE id = $1")
        .bind(&account_id)
        .execute(&mut **transaction)
        .await?;

    sqlx::query("DELETE FROM ps_auths WHERE id = $1")
        .bind(&account_id)
        .execute(&mut **transaction)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

// get accounts method
pub async fn get_all_pjsip_accounts(
    pool: &PgPool,
) -> Result<Vec<PjsipRealtimeAccountWithId>, sqlx::Error> {
    let query = "
        SELECT 
            id,
            username,
            password,
            transport,
            context,
            from_domain,
            from_user,
            created_at,
            updated_at
        FROM pjsip_realtime_accounts
        ORDER BY created_at DESC
    ";

    let rows = sqlx::query(query).fetch_all(pool).await?;

    let mut accounts = Vec::new();
    for row in rows {
        let transport_str: String = row.get("transport");
        let transport = match transport_str.to_lowercase().as_str() {
            "udp" => TransportType::Udp,
            "tcp" => TransportType::Tcp,
            "tls" => TransportType::Tls,
            "ws" => TransportType::Ws,
            "wss" => TransportType::Wss,
            _ => TransportType::Udp, // default fallback
        };

        accounts.push(PjsipRealtimeAccountWithId {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            transport,
            context: row.get("context"),
            from_domain: row.get("from_domain"),
            from_user: row.get("from_user"),
            rtp_timeout: None,
            rtp_timeout_hold: None,
            created_at: row.get::<chrono::NaiveDateTime, _>("created_at").and_utc(),
            updated_at: row.get::<chrono::NaiveDateTime, _>("updated_at").and_utc(),
        });
    }

    Ok(accounts)
}
