#[cfg(test)]
mod tests {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::Value;
    use sqlx::PgPool;
    use tower::ServiceExt;

    use crate::{
        AppState, infrastructure::models::pjsip_realtime::account::PjsipRealtimeAccountWithId,
        restapi::routes::pjsip_realtime_router::pjsip_realtime_router,
    };

    async fn setup_test_app() -> Router {
        // Note: In real tests, you would set up a test database
        // For now, we'll use the actual database pool
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        let state = AppState { pjsip_db: pool };
        pjsip_realtime_router(state)
    }

    #[tokio::test]
    async fn test_get_pjsip_accounts_success() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .method("GET")
            .uri("/accounts")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let accounts: Vec<PjsipRealtimeAccountWithId> = serde_json::from_slice(&body).unwrap();

        // Basic structure validation
        for account in accounts {
            assert!(!account.id.is_empty());
            assert!(!account.username.is_empty());
            assert!(!account.context.is_empty());
            assert!(!account.from_domain.is_empty());
            assert!(!account.from_user.is_empty());
        }
    }

    #[tokio::test]
    async fn test_get_pjsip_accounts_empty_database() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .method("GET")
            .uri("/accounts")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        // Should still return 200 even if no accounts exist
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let accounts: Vec<PjsipRealtimeAccountWithId> = serde_json::from_slice(&body).unwrap();

        // Could be empty or contain existing data
        // This test validates the response structure
        println!("Found {} accounts in database", accounts.len());
    }

    #[tokio::test]
    async fn test_accounts_response_format() {
        let app = setup_test_app().await;

        let request = Request::builder()
            .method("GET")
            .uri("/accounts")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json_value: Value = serde_json::from_slice(&body).unwrap();

        // Verify it's an array
        assert!(json_value.is_array());

        if let Some(array) = json_value.as_array() {
            if !array.is_empty() {
                // Check first account has required fields
                let first_account = &array[0];
                assert!(first_account.get("id").is_some());
                assert!(first_account.get("username").is_some());
                assert!(first_account.get("password").is_some());
                assert!(first_account.get("transport").is_some());
                assert!(first_account.get("context").is_some());
                assert!(first_account.get("from_domain").is_some());
                assert!(first_account.get("from_user").is_some());
                assert!(first_account.get("created_at").is_some());
                assert!(first_account.get("updated_at").is_some());
            }
        }
    }
}
