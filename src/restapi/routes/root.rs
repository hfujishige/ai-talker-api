use axum::{Router, routing::get};

use crate::AppState;
use crate::restapi::handlers::heart_beat_handler::heart_beat;
use crate::restapi::routes::pjsip_realtime_router::pjsip_realtime_router;

pub fn create_router(state: AppState) -> Router {
    Router::new().route("/", get(heart_beat)).nest(
        "/api/v1/pjsip_realtime",
        pjsip_realtime_router(state.clone()),
    )
}
