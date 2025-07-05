use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

pub async fn heart_beat() -> impl IntoResponse {
    let response = "respond.";
    (StatusCode::OK, Json(response))
}
