#[cfg(test)]
mod tests {
    use axum::{
        Router,
        body::{Body, Bytes},
        http::response::Response,
        http::{Request, StatusCode},
    };
    use dotenvy::from_filename;
    use http_body_util::BodyExt;
    use serde_json::Value;
    use serial_test::serial;
    use sqlx::{Error, PgPool, Pool, Postgres};
    use tower::ServiceExt;

    use crate::{
        AppState,
        create_pjsip_pool,
        infrastructure::models::pjsip_realtime::{
            account::PjsipRealtimeAccountWithId, enums::pjsip_endpoint_enums::TransportType,
        },
        restapi::routes::pjsip_realtime_router::pjsip_realtime_router,
    };

    async fn setup_test_app() -> Router {
        // Load test environment variables
        from_filename(".env.test").ok();

        // Create database pool using the same method as other tests
        let create_pjsip_pool_result: Result<Pool<Postgres>, Error> = create_pjsip_pool().await;
        let pool: PgPool = match create_pjsip_pool_result {
            Ok(pool) => pool,
            Err(e) => {
                tracing::error!("Failed to create PJSIP database connection pool: {}", e);
                panic!("Failed to create PJSIP database connection pool");
            }
        };

        // Clean up any existing test data
        let _ = sqlx::query("DELETE FROM pjsip_realtime_accounts WHERE id = '01HX1234567890ABCDEFGHIJK9' OR username = 'external_id_test_user'")
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM ps_auths WHERE id = '01HX1234567890ABCDEFGHIJK9' OR username = 'external_id_test_user'")
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM ps_aors WHERE id = '01HX1234567890ABCDEFGHIJK9' OR username = 'external_id_test_user'")
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM ps_endpoints WHERE id = '01HX1234567890ABCDEFGHIJK9' OR username = 'external_id_test_user'")
            .execute(&pool)
            .await;

        let state: AppState = AppState { pjsip_db: pool };
        pjsip_realtime_router(state)
    }

    async fn cleanup_test_data() {
        // Load test environment variables
        from_filename(".env.test").ok();

        // Create database pool using the same method as other tests
        let create_pjsip_pool_result: Result<Pool<Postgres>, Error> = create_pjsip_pool().await;
        if let Ok(pool) = create_pjsip_pool_result {
            let _ = sqlx::query("DELETE FROM pjsip_realtime_accounts WHERE id = '01HX1234567890ABCDEFGHIJK9' OR username = 'external_id_test_user'")
                .execute(&pool)
                .await;
        }
    }

    #[serial]
    #[tokio::test]
    async fn test_create_pjsip_account_with_external_id_success() {
        // Clean up any leftover test data
        cleanup_test_data().await;

        let app: Router = setup_test_app().await;

        let request_body: Value = serde_json::json!({
            "id": "01HX1234567890ABCDEFGHIJK9",
            "username": "external_id_test_user",
            "password": "test_password_123",
            "transport": "udp",
            "context": "from-sipproxy",
            "from_domain": "test.example.com",
            "from_user": "external_id_test_user"
        });

        let request: Request<Body> = Request::builder()
            .method("POST")
            .uri("/accounts_with_id")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
            .unwrap();

        let response: Response<Body> = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body: Bytes = response.into_body().collect().await.unwrap().to_bytes();
        let created_account: PjsipRealtimeAccountWithId = serde_json::from_slice(&body).unwrap();

        assert_eq!(created_account.id, "01HX1234567890ABCDEFGHIJK9");
        assert_eq!(created_account.username, "external_id_test_user");
        assert_eq!(created_account.transport, TransportType::Udp);
        assert_eq!(
            created_account.context,
            request_body["context"].as_str().unwrap()
        );
        assert_eq!(
            created_account.from_domain,
            request_body["from_domain"].as_str().unwrap()
        );
        assert_eq!(
            created_account.from_user,
            request_body["from_user"].as_str().unwrap()
        );
    }

    #[tokio::test]
    async fn test_create_pjsip_account_with_external_id_empty_id() {
        let app: Router = setup_test_app().await;

        let request_body: Value = serde_json::json!({
            "id": "",
            "username": "test_user_empty_id",
            "password": "test_password_123",
            "transport": "udp",
            "context": "from-sipproxy",
            "from_domain": "test.example.com",
            "from_user": "test_user_empty_id"
        });

        let request: Request<Body> = Request::builder()
            .method("POST")
            .uri("/accounts_with_id")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
            .unwrap();

        let response: Response<Body> = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body: Bytes = response.into_body().collect().await.unwrap().to_bytes();
        let error_response: Value = serde_json::from_slice(&body).unwrap();

        assert!(error_response.get("error").is_some());
        assert!(
            error_response["error"]
                .as_str()
                .unwrap()
                .contains("ID cannot be empty")
        );
    }

    #[tokio::test]
    async fn test_create_pjsip_account_with_external_id_duplicate_id() {
        let app: Router = setup_test_app().await;

        let request_body: Value = serde_json::json!({
            "id": "01HX1234567890DUPLICATE_ID",
            "username": "duplicate_test_user_1",
            "password": "test_password_123",
            "transport": "udp",
            "context": "from-sipproxy",
            "from_domain": "test.example.com",
            "from_user": "duplicate_test_user_1"
        });

        // First request - should succeed
        let request1 = Request::builder()
            .method("POST")
            .uri("/accounts_with_id")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
            .unwrap();

        let response1 = app.clone().oneshot(request1).await.unwrap();

        // May succeed or fail depending on existing data, but we proceed to test duplicate

        // Second request with same ID - should fail
        let request_body2 = serde_json::json!({
            "id": "01HX1234567890DUPLICATE_ID",
            "username": "duplicate_test_user_2",
            "password": "test_password_456",
            "transport": "udp",
            "context": "users",
            "from_domain": "test2.example.com",
            "from_user": "duplicate_test_user_2"
        });

        let request2: Request<Body> = Request::builder()
            .method("POST")
            .uri("/accounts_with_id")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&request_body2).unwrap()))
            .unwrap();

        let response2: Response<Body> = app.oneshot(request2).await.unwrap();

        // Should return conflict status if the first request succeeded
        if response1.status() == StatusCode::CREATED {
            assert_eq!(response2.status(), StatusCode::CONFLICT);
        }
    }

    #[tokio::test]
    async fn test_create_pjsip_account_with_external_id_invalid_json() {
        let app: Router = setup_test_app().await;

        let invalid_json: &'static str = "{invalid_json}";

        let request: Request<Body> = Request::builder()
            .method("POST")
            .uri("/accounts_with_id")
            .header("content-type", "application/json")
            .body(Body::from(invalid_json))
            .unwrap();

        let response: Response<Body> = app.oneshot(request).await.unwrap();

        // Should return bad request for invalid JSON
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
