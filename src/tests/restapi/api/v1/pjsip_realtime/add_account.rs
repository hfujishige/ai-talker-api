use crate::{AppState, create_pjsip_pool};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use dotenvy::from_filename;
use http_body_util::BodyExt; // for `collect`
use serde_json::json;
use serial_test::serial;
use sqlx::MySqlPool;
use tower::ServiceExt;

#[tokio::test]
#[serial]
async fn test_add_user() {
    /*
    curl -X POST -H "Content-Type: application/json" \
      -d '{"login_id" : "test_user" , "email" : "test_user@example.com", "password": "test_password", "name": "Test User" }' \
      http://localhost:3000/api/v1/pjsip_realtime/add_user
    */
    // Config file
    from_filename(".env.test").ok();

    // Create a test server
    let pjsip_db: MySqlPool = create_pjsip_pool().await;
    let state = AppState { pjsip_db };
    let app = crate::restapi::routes::root::create_router(state.clone());

    // Define the JSON payload
    let payload = json!({
        "username": "test_user",
        "password": "test_password",
        "transport": "TransportUdp",
        "context": "default",
        "from_domain": "default_user",
        "from_user": "default_user",
    });

    // Create a request to the endpoint
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/pjsip_realtime/accounts")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    // Send the request and get the response
    let response = app.oneshot(request).await.unwrap();

    // Assert the response status code
    assert_eq!(response.status(), StatusCode::CREATED);

    // Assert the response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Assert response JSON and payload
    assert_eq!(response_json["username"], payload["username"]);
    assert_eq!(response_json["password"], payload["password"]);
    assert_eq!(response_json["transport"], payload["transport"]);
    assert_eq!(response_json["context"], payload["context"]);
    assert_eq!(response_json["from_domain"], payload["from_domain"]);
    assert_eq!(response_json["from_user"], payload["from_user"]);

    // Clean up test data
    let pjsip_db2: MySqlPool = create_pjsip_pool().await;
    let mut transaction = pjsip_db2.begin().await.unwrap();
    let delete_ps_auths_sql = r#"DELETE FROM ps_auths;"#;
    let delete_ps_aors_sql = r#"DELETE FROM ps_aors;"#;
    let delete_ps_endpoints_sql = r#"DELETE FROM ps_endpoints;"#;
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
