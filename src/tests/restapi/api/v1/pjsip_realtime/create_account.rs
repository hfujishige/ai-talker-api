use crate::tests::restapi::api::v1::pjsip_realtime::account_helper::reset_pjsip_realtime_database;
use crate::{AppState, create_pjsip_pool};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use dotenvy::from_filename;
use http_body_util::BodyExt; // for `collect`
use serde_json::Value;
use serial_test::serial;
use sqlx::{Error, PgPool, Pool, Postgres};
use tower::ServiceExt;

#[serial]
#[tokio::test]
async fn test_create_pjsip_realtime_account() {
    /*
    curl -X POST -H "Content-Type: application/json" \
      -d '{"username" : "test_user" , ... , "from_user": "Test default_user" }' \
      http://localhost:3000/api/v1/pjsip_realtime/accounts/
    */
    // Config file
    from_filename(".env.test").ok();

    // Create a test server
    // let pjsip_db: PgPool = create_pjsip_pool().await;
    let create_pjsip_pool_result: Result<Pool<Postgres>, Error> = create_pjsip_pool().await;
    let pjsip_db: PgPool = match create_pjsip_pool_result {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to create PJSIP database connection pool: {}", e);
            panic!("Failed to create PJSIP database connection pool");
        }
    };
    let state = AppState { pjsip_db };
    let app = crate::restapi::routes::root::create_router(state.clone());

    // reset database before test
    reset_pjsip_realtime_database(&state.pjsip_db).await;

    // Define the JSON payload
    let payload: Value = serde_json::json!({
        "username": "test_user",
        "password": "test_password",
        "transport": "udp",
        "context": "from-sipproxy",
        "from_domain": "default_domain",
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
    println!(
        "check transport: request: {}, response: {}",
        payload["transport"], response_json["transport"]
    );
    assert_eq!(response_json["username"], payload["username"]);
    assert_eq!(response_json["password"], payload["password"]);
    assert_eq!(response_json["transport"], payload["transport"]);
    assert_eq!(response_json["context"], payload["context"]);
    assert_eq!(response_json["from_domain"], payload["from_domain"]);
    assert_eq!(response_json["from_user"], payload["from_user"]);

    // reset database after test
    reset_pjsip_realtime_database(&state.pjsip_db).await;

    // Clean up test data
    // let create_pjsip_pool_result2: Result<Pool<Postgres>, Error> = create_pjsip_pool().await;
    // let pjsip_db2: PgPool = match create_pjsip_pool_result2 {
    //     Ok(pool) => pool,
    //     Err(e) => {
    //         tracing::error!("Failed to create PJSIP database connection pool: {}", e);
    //         panic!("Failed to create PJSIP database connection pool");
    //     }
    // };

    // let mut transaction = pjsip_db2.begin().await.unwrap();
    // let delete_ps_auths_sql = r#"DELETE FROM ps_auths;"#;
    // let delete_ps_aors_sql = r#"DELETE FROM ps_aors;"#;
    // let delete_ps_endpoints_sql = r#"DELETE FROM ps_endpoints;"#;
    // sqlx::query(delete_ps_auths_sql)
    //     .execute(&mut *transaction)
    //     .await
    //     .unwrap();
    // sqlx::query(delete_ps_aors_sql)
    //     .execute(&mut *transaction)
    //     .await
    //     .unwrap();
    // sqlx::query(delete_ps_endpoints_sql)
    //     .execute(&mut *transaction)
    //     .await
    //     .unwrap();

    // transaction.commit().await.unwrap();
    // pjsip_db2.close().await;
    // tracing::info!("Delete test data successfully.");
}
