use axum::{
    Router,
    routing::{delete, get, post},
};
// use axum::extract::State;
use crate::AppState;
use crate::restapi::handlers::pjsip_realtime_handler::{
    create_pjsip_account_handler, create_pjsip_account_with_external_id_handler,
    delete_pjsip_account_handler, get_pjsip_accounts_handler,
};

pub fn pjsip_realtime_router(state: AppState) -> Router {
    // base path is /api/v1/pjsip_realtime/
    Router::new()
        .route("/accounts", get(get_pjsip_accounts_handler))
        .route("/accounts", post(create_pjsip_account_handler))
        .route(
            "/accounts_with_id",
            post(create_pjsip_account_with_external_id_handler),
        )
        .route(
            "/accounts/{account_id}",
            delete(delete_pjsip_account_handler),
        )
        .with_state(state)
}
