use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::Value;

use crate::AppState;
use crate::application::repository::pjsip_realtime::{
    create_udp_pjsip_account, delete_pjsip_account,
};
use crate::infrastructure::models::pjsip_realtime::{
    account::PjsipRealtimeAccount, enums::pjsip_endpoint_enums::TransportType,
};

pub async fn get_pjsip_accounts_handler(_: State<AppState>) -> impl IntoResponse {
    // Here you would typically query the database using `state.pjsip_db`
    let users = vec!["user1", "user2", "user3"];
    (StatusCode::OK, Json(users))
}

pub async fn create_pjsip_account_handler(
    state: State<AppState>,
    Json(payload): Json<PjsipRealtimeAccount>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let account = payload;

    // payload.transportがTransportTypeのいずれかの値であることを確認し、個別の処理に振り分け（スタブ）
    match account.transport {
        TransportType::TransportUdp => {
            // UDP用の処理（スタブ）
            if let Err(e) = create_udp_pjsip_account(state.clone(), &account).await {
                eprintln!("Failed to create UDP account: {:?}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!("Failed to create UDP account")),
                ));
            }
        }
        TransportType::TransportTcp => {
            // TCP用の処理（スタブ）
            println!("Creating TCP account: {:?}", account.username);
        }
        TransportType::TransportTls => {
            // TLS用の処理（スタブ）
            println!("Creating TLS account: {:?}", account.username);
        }
        TransportType::TransportWs => {
            // WebSocket用の処理（スタブ）
            println!("Creating WS account: {:?}", account.username);
        }
        TransportType::TransportWss => {
            // Secure WebSocket用の処理（スタブ）
            println!("Creating WSS account: {:?}", account.username);
        } //_ => {} // もし他のバリアントが存在する場合はここで対応
    }
    // set account type by payload.transport(TransportType)
    let ret_acc: PjsipRealtimeAccount = account.clone();
    // (StatusCode::OK,
    //  Json(format!("name: {}, password: {}, transport: {}",
    //               ret_acc.username, ret_acc.password, ret_acc.transport)))
    Ok((StatusCode::CREATED, Json(ret_acc)))
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
