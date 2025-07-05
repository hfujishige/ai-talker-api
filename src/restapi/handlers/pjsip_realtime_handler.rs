use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::Value;

use crate::AppState;
use crate::application::repository::pjsip_realtime::{
    create_udp_pjsip_account, delete_pjsip_account, get_pjsip_accounts,
};
use crate::infrastructure::models::pjsip_realtime::{
    account::{
        PjsipRealtimeAccount, PjsipRealtimeAccountWithExternalId, PjsipRealtimeAccountWithId,
    },
    enums::pjsip_endpoint_enums::TransportType,
};

pub async fn get_pjsip_accounts_handler(state: State<AppState>) -> impl IntoResponse {
    match get_pjsip_accounts(state).await {
        Ok(accounts) => (StatusCode::OK, Json(accounts)),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<PjsipRealtimeAccountWithId>::new()),
            )
        }
    }
}

pub async fn create_pjsip_account_handler(
    state: State<AppState>,
    Json(payload): Json<PjsipRealtimeAccount>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let account = payload;
    let account_id: Option<String> = None;
    // payload.transportがTransportTypeのいずれかの値であることを確認し、個別の処理に振り分け（スタブ）
    match create_pjsip_account(state.clone(), account_id, &account).await {
        Ok((status, json_response)) => {
            // ここでjson_responseがserde_json::Value型であることを想定
            // responseは (StatusCode, Json<Value>) 型を想定
            let id = json_response
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            println!("Created account {} with ID: {}", account.username, id);

            // PjsipRealtimeAccountWithId型の変数を作成してidと引数データをまとめる
            let now = chrono::Utc::now();
            let response_account = PjsipRealtimeAccountWithId {
                id: String::from(id),
                username: account.username,
                password: account.password,
                context: account.context,
                transport: account.transport,
                from_domain: account.from_domain,
                from_user: account.from_user,
                created_at: now,
                updated_at: now,
            };
            Ok((status, Json(response_account)))
        }
        Err((status, json_response)) => {
            eprintln!("Failed to create account: {:?}", json_response);
            match status {
                // duplicate key error
                StatusCode::CONFLICT => Err((
                    StatusCode::CONFLICT,
                    Json(serde_json::json!(
                        "Account with this ID or username already exists"
                    )),
                )),
                _ => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!("Failed to create account")),
                )),
            }
        }
    }
}

pub async fn create_pjsip_account_with_external_id_handler(
    state: State<AppState>,
    Json(payload): Json<PjsipRealtimeAccountWithExternalId>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Validate the pjsip realtime account ID (should be a valid ULID or UUID format)
    if payload.id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "ID cannot be empty"})),
        ));
    }

    // Validate username
    if payload.username.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Username cannot be empty"})),
        ));
    }

    let new_account_id: Option<String> = payload.id.clone().into();
    let account: PjsipRealtimeAccount = PjsipRealtimeAccount {
        username: payload.username,
        password: payload.password,
        transport: payload.transport,
        context: payload.context,
        from_domain: payload.from_domain,
        from_user: payload.from_user,
    };
    match create_pjsip_account(state.clone(), new_account_id, &account).await {
        Ok((status, json_response)) => {
            // ここでjson_responseがserde_json::Value型であることを想定
            // responseは (StatusCode, Json<Value>) 型を想定
            let id = json_response
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            println!("Created account {} with ID: {}", account.username, id);

            // PjsipRealtimeAccountWithId型の変数を作成してidと引数データをまとめる
            let now = chrono::Utc::now();
            let response_account = PjsipRealtimeAccountWithId {
                id: String::from(id),
                username: account.username,
                password: account.password,
                context: account.context,
                transport: account.transport,
                from_domain: account.from_domain,
                from_user: account.from_user,
                created_at: now,
                updated_at: now,
            };
            Ok((status, Json(response_account)))
        }
        Err((status, json_response)) => {
            eprintln!("Failed to create account: {:?}", json_response);
            match status {
                // duplicate key error
                StatusCode::CONFLICT => Err((
                    StatusCode::CONFLICT,
                    Json(serde_json::json!(
                        "Account with this ID or username already exists"
                    )),
                )),
                _ => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!("Failed to create account")),
                )),
            }
        }
    }
}

pub async fn delete_pjsip_account_handler(
    state: State<AppState>,
    Path(account_id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // TODO validate account_id

    if let Err(e) = delete_pjsip_account(state.clone(), account_id).await {
        eprintln!("Failed to delete account: {:?}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!("Failed to delete account")),
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

// private functions for handling pjsip accounts
async fn create_pjsip_account(
    state: State<AppState>,
    account_id: Option<String>,
    account: &PjsipRealtimeAccount,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // payload.transportがTransportTypeのいずれかの値であることを確認し、個別の処理に振り分け（スタブ）
    match account.transport {
        TransportType::Udp => {
            // UDP用の処理（スタブ）
            create_udp_pjsip_account(state.clone(), account_id, &account).await
        }
        TransportType::Tcp => {
            // TCP用の処理（スタブ）
            println!("Creating TCP account: {:?}", account.username);
            Err((
                StatusCode::NOT_IMPLEMENTED,
                Json(serde_json::json!("TCP transport not implemented")),
            ))
        }
        TransportType::Tls => {
            // TLS用の処理（スタブ）
            println!("Creating TLS account: {:?}", account.username);
            Err((
                StatusCode::NOT_IMPLEMENTED,
                Json(serde_json::json!("TLS transport not implemented")),
            ))
        }
        TransportType::Ws => {
            // WebSocket用の処理（スタブ）
            println!("Creating WS account: {:?}", account.username);
            Err((
                StatusCode::NOT_IMPLEMENTED,
                Json(serde_json::json!("WS transport not implemented")),
            ))
        }
        TransportType::Wss => {
            // Secure WebSocket用の処理（スタブ）
            println!("Creating WSS account: {:?}", account.username);
            Err((
                StatusCode::NOT_IMPLEMENTED,
                Json(serde_json::json!("WSS transport not implemented")),
            ))
        } //_ => {} // もし他のバリアントが存在する場合はここで対応
    }
}
