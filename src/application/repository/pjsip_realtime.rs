use crate::AppState;
use crate::infrastructure::models::errors::registration_error::RegistrationError;
use crate::infrastructure::models::pjsip_realtime::enums::{
    pjsip_auth_enums::AuthType,
    pjsip_endpoint_enums::{DtmfMode, MediaEncryption, TransportType},
    pjsip_realtime_common_enums::TurnOnOff,
};
use crate::infrastructure::models::pjsip_realtime::{
    account::{PjsipRealtimeAccount, PjsipRealtimeAccountWithId},
    sip_udp::{PsAorForUdp, PsAuthForUdp, PsEndpointForUdp},
};
use crate::infrastructure::repository::pjsip_realtime_repository::{
    exec_delete_pjsip_account, exec_insert_udp_pjsip_account, get_all_pjsip_accounts,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::Value;
use ulid::Ulid;

pub async fn create_udp_pjsip_account(
    state: State<AppState>,
    account_id: Option<String>,
    account: &PjsipRealtimeAccount,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    println!("TODO create_udp_account validation here.");

    // validation

    // common
    // If account_id is provided, use it; otherwise, generate a new ULID
    let new_account_id: String = match account_id {
        Some(id) => id,
        None => Ulid::new().to_string(),
    };

    // let account_ulid: String = new_account_id;
    let pjsip_account: PjsipRealtimeAccountWithId = PjsipRealtimeAccountWithId {
        id: new_account_id.clone(),
        username: account.username.clone(),
        password: account.password.clone(),
        context: account.context.clone(),
        transport: account.transport.clone(),
        from_domain: account.from_domain.clone(),
        from_user: account.from_user.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // ps_auth
    let ps_auth: PsAuthForUdp = PsAuthForUdp {
        id: new_account_id.clone(),
        auth_type: AuthType::Userpass, // ここは必ず AuthType の値
        username: account.username.clone(),
        password: account.password.clone(),
    };

    // ps_aor
    let ps_aor: PsAorForUdp = PsAorForUdp {
        id: new_account_id.clone(),
        max_contacts: 5,
        remove_existing: TurnOnOff::Yes,
        remove_unavailable: TurnOnOff::Yes,
        default_expiration: 60,
        minimum_expiration: 60,
        maximum_expiration: 90,
        qualify_frequency: 10,
        qualify_timeout: 9,
    };

    // ps_endpoint
    let ps_endpoint: PsEndpointForUdp = PsEndpointForUdp {
        id: new_account_id.clone(),
        transport: TransportType::Udp,
        aors: new_account_id.clone(),
        auth: new_account_id.clone(),
        context: account.context.clone(),
        disallow: String::from("all"),
        allow: String::from("ulaw,opus"),
        direct_media: TurnOnOff::No,
        dtmf_mode: DtmfMode::Auto,
        force_rport: TurnOnOff::Yes,
        rewrite_contact: TurnOnOff::Yes,
        rtp_ipv6: TurnOnOff::Yes,
        rtp_symmetric: TurnOnOff::Yes,
        media_encryption: MediaEncryption::No,
        from_domain: account.from_domain.clone(),
        from_user: account.from_user.clone(),
    };

    // register account in database
    let mut transaction: sqlx::Transaction<'static, sqlx::Postgres> =
        state.pjsip_db.begin().await.map_err(|e: sqlx::Error| {
            let error_message: String = format!("Failed to begin transaction: {}", e);
            let value: Value = serde_json::json!({ "error": error_message });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(value))
        })?;

    let result: Result<StatusCode, RegistrationError> = exec_insert_udp_pjsip_account(
        &mut transaction,
        &pjsip_account,
        &ps_auth,
        &ps_aor,
        &ps_endpoint,
    )
    .await;
    match result {
        Ok(_) => {
            transaction.commit().await.map_err(|e| {
                let error_message = format!("Failed to commit transaction: {}", e);
                let value: Value = serde_json::json!({ "error": error_message });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(value))
            })?;
            let value: Value = serde_json::json!({"id": new_account_id});
            Ok((StatusCode::CREATED, Json(value)))
        }
        Err(e) => {
            let _ = transaction.rollback().await;
            let error_message = format!("Failed to create UDP account: {}", e);
            let value: Value = serde_json::json!({ "error": error_message });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(value)))
        }
    }
}

pub async fn delete_pjsip_account(
    state: State<AppState>,
    account_id: String,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // repository delete
    let mut transaction = state.pjsip_db.begin().await.map_err(|e| {
        let error_message = format!("Failed to begin transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": error_message })),
        )
    })?;

    let result = exec_delete_pjsip_account(&mut transaction, account_id).await;
    match result {
        Ok(_) => {
            transaction.commit().await.map_err(|e| {
                let error_message = format!("Failed to commit transaction: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": error_message })),
                )
            })?;
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            let _ = transaction.rollback().await;
            let error_message = format!("Failed to delete account: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": error_message })),
            ))
        }
    }
}

pub async fn get_accounts(_: State<AppState>) -> impl IntoResponse {
    // Here you would typically query the database using `state.pjsip_db`
    let users = vec!["user1", "user2", "user3"];
    (StatusCode::OK, Json(users))
}

pub async fn get_pjsip_accounts(
    state: State<AppState>,
) -> Result<Vec<PjsipRealtimeAccountWithId>, sqlx::Error> {
    get_all_pjsip_accounts(&state.pjsip_db).await
}
