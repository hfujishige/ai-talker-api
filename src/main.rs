mod application;
mod infrastructure;
mod restapi;
mod tests;

use axum::Router;

use dotenvy::dotenv;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use std::env;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    pjsip_db: MySqlPool,
}

async fn create_pjsip_pool() -> MySqlPool {
    let db_scheme: String = env::var("PJSIP_DB_SCHEME").expect("PJSIP_DB_SCHEME must be set");
    let db_user: String = env::var("PJSIP_DB_USER").expect("PJSIP_DB_USER must be set");
    let db_password: String = env::var("PJSIP_DB_PWD").expect("PJSIP_DB_PASSWORD must be set");
    let db_host: String = env::var("PJSIP_DB_HOST").expect("PJSIP_DB_HOST must be set");
    let db_port: String = env::var("PJSIP_DB_PORT").expect("PJSIP_DB_PORT must be set");
    let db_catalog: String = env::var("PJSIP_DB_CATALOG").expect("PJSIP_DB_CATALOG must be set");
    let db_max_conn: u32 = env::var("PJSIP_DB_POOL_SIZE")
        .expect("PJSIP_DB_POOL_SIZE must be set")
        .parse()
        .unwrap();

    let url: String = format!(
        "{}://{}:{}@{}:{}/{}",
        db_scheme, db_user, db_password, db_host, db_port, db_catalog
    );

    println!("Connecting to PJSIP database at: {}", url);
    MySqlPoolOptions::new()
        .max_connections(db_max_conn)
        .connect(&url)
        .await
        .expect("Failed to connect to PJSIP database")
}

#[tokio::main]
async fn main() {
    // initialize tracing(logging)
    tracing_subscriber::fmt::init();
    // TODO tracing configurations

    // configurations
    dotenv().ok();

    let pjsip_db: MySqlPool = create_pjsip_pool().await;

    let state = AppState { pjsip_db };

    let router: Router = restapi::routes::root::create_router(state.clone());
    // let router: Router = Router::new().route("/", get(hello_world)).nest(
    //     "/api/v1/pjsip_realtime",
    //     pjsip_realtime_router(state.clone()),
    // );

    let listen_ipv4: String = env::var("LISTEN_IPV4").expect("LISTEN_IPV4 must be set");
    let listen_port_v4: String = env::var("LISTEN_PORT_V4").expect("LISTEN_PORT_V4 must be set");
    // RDBMS
    let listen_addr: String = format!("{}:{}", listen_ipv4, listen_port_v4);
    let listener: TcpListener = tokio::net::TcpListener::bind(&listen_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

// async fn hello_world() -> impl IntoResponse {
//     let response = "Hello, world!";
//     (StatusCode::OK, Json(response))
// }
