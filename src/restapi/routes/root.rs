use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};

use crate::AppState;
use crate::restapi::routes::pjsip_realtime_router::pjsip_realtime_router;

// Define your hello_world handler and pjsip_realtime_router if not already defined elsewhere

pub fn create_router(state: AppState) -> Router {
    Router::new().route("/", get(heart_beat)).nest(
        "/api/v1/pjsip_realtime",
        pjsip_realtime_router(state.clone()),
    )
}

async fn heart_beat() -> impl IntoResponse {
    let response = "respond.";
    (StatusCode::OK, Json(response))
}
