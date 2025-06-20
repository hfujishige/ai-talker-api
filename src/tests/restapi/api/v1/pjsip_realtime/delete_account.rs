use crate::infrastructure::models::pjsip_realtime::account::PjsipRealtimeAccountWithId;
use crate::{AppState, create_pjsip_pool};

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use dotenvy::from_filename;
use http_body_util::BodyExt; // for `collect`
use serde_json::{Value, json};
use serial_test::serial;
use sqlx::MySqlPool;
use tower::ServiceExt;

#[serial]
#[tokio::test]
async fn test_delete_pjsip_realtime_account() {
    /*
    curl -X POST -H "Content-Type: application/json" \
      -d '{"username" : "test_user" , ... , "from_user": "Test default_user" }' \
      http://localhost:3000/api/v1/pjsip_realtime/accounts/
    */
    // Config file
    from_filename(".env.test").ok();

    // Create a test server
    let pjsip_db: MySqlPool = create_pjsip_pool().await;
    let state: AppState = AppState { pjsip_db };
    let app: Router = crate::restapi::routes::root::create_router(state.clone());

    // Define the JSON payload
    let payload: String = json!({
        "username": "test_user",
        "password": "test_password",
        "transport": "TransportUdp",
        "context": "default",
        "from_domain": "default_domain",
        "from_user": "default_user",
    })
    .to_string();

    // Create a request to the endpoint
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/pjsip_realtime/accounts")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(Body::from(payload))
        .unwrap();

    // Send the create request and get the response
    let response = app.oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let response_json: Value = serde_json::from_slice(&body).unwrap();

    let account: PjsipRealtimeAccountWithId =
        serde_json::from_value(response_json.clone()).unwrap();
    let account_id: String = account.id.clone();

    // Send the delete request and assert the response status code
    let del_app = crate::restapi::routes::root::create_router(state.clone());
    let del_req_params: String = json!({"account_id": account_id}).to_string();
    let del_request = Request::builder()
        .method("DELETE")
        .uri(format!("/api/v1/pjsip_realtime/accounts/{}", account_id))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(Body::from(del_req_params))
        .unwrap();
    let del_response = del_app.oneshot(del_request).await.unwrap();
    // Assert response JSON and payload
    assert_eq!(del_response.status(), StatusCode::NO_CONTENT);

    // Clean up test data
    let pjsip_db2: MySqlPool = create_pjsip_pool().await;
    let mut transaction: sqlx::Transaction<'static, sqlx::MySql> = pjsip_db2.begin().await.unwrap();
    let delete_ps_auths_sql: &'static str = r#"DELETE FROM ps_auths;"#;
    let delete_ps_aors_sql: &'static str = r#"DELETE FROM ps_aors;"#;
    let delete_ps_endpoints_sql: &'static str = r#"DELETE FROM ps_endpoints;"#;
    sqlx::query(delete_ps_auths_sql)
        .execute(&mut *transaction)
        .await
        .unwrap();
    sqlx::query(delete_ps_aors_sql)
        .execute(&mut *transaction)
        .await
        .unwrap();
    sqlx::query(delete_ps_endpoints_sql)
        .execute(&mut *transaction)
        .await
        .unwrap();

    transaction.commit().await.unwrap();
    pjsip_db2.close().await;
    tracing::info!("Delete test data successfully.");
}
