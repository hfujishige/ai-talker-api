        use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::Value;

use crate::AppState;
use crate::application::repository::pjsip_realtime::{
    create_udp_pjsip_account, delete_pjsip_account,
};
use crate::infrastructure::models::pjsip_realtime::{
    account::{PjsipRealtimeAccount, PjsipRealtimeAccountWithId},
    enums::pjsip_endpoint_enums::TransportType,
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
            let result = create_udp_pjsip_account(state.clone(), &account).await;
            match result {
                Ok(response) => {
                    // ここでjson_responseがserde_json::Value型であることを想定
                    // responseは (StatusCode, Json<Value>) 型を想定
                    let (status, json_response) = response;
                    let id = json_response
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    println!("Created account {} with ID: {}", account.username, id);

                    // PjsipRealtimeAccount型の変数を作成してidと引数データをまとめる
                    let response_account = PjsipRealtimeAccountWithId {
                        id: String::from(id),
                        username: account.username,
                        password: account.password,
                        context: account.context,
                        transport: account.transport,
                        from_domain: account.from_domain,
                        from_user: account.from_user,
                    };
                    return Ok((status, Json(response_account)));
                }
                Err(e) => {
                    println!("Failed to create UDP account: {:?}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!("Failed to create UDP account")),
                    ));
                }
            }
            // if let Err(e) = create_udp_pjsip_account(state.clone(), &account).await {
            //     eprintln!("Failed to create UDP account: {:?}", e);
            //     return Err((
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         Json(serde_json::json!("Failed to create UDP account")),
            //     ));
            // }
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
    // ret_accのjsonデータに含まれるidと、payloadの各種値を使って新しいPjsipRealtimeAccountを作成する
    // let ret_id =
    // let ret_acc = PjsipRealtimeAccount {
    //     id: ret_,
    //     username: account.username,
    //     password: account.password,
    //     transport: account.transpo,
    //     context: account.context,
    //     from_domain: account.from_domain,
    //     from_user: account.from_user,
    //     // 他のフィールドがあればここに追加
    //     ..Default::default()
    // };
    // (StatusCode::OK,
    //  Json(format!("name: {}, password: {}, transport: {}",
    //               ret_acc.username, ret_acc.password, ret_acc.transport)))
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!("Not implemented")),
    ))
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
