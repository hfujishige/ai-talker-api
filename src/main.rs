mod application;
mod infrastructure;
mod restapi;

#[cfg(test)]
mod tests;

use axum::Router;
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use tokio::net::TcpListener;
use tracing::{debug, info};

#[derive(Clone)]
struct AppState {
    pjsip_db: PgPool,
}

async fn create_pjsip_pool() -> Result<PgPool, sqlx::Error> {
    let db_scheme: String = env::var("PJSIP_DB_SCHEME").expect("PJSIP_DB_SCHEME must be set");
    let db_user: String = env::var("PJSIP_DB_USER").expect("PJSIP_DB_USER must be set");
    let db_password: String = env::var("PJSIP_DB_PWD").expect("PJSIP_DB_PWD must be set");
    let db_host: String = env::var("PJSIP_DB_HOST").expect("PJSIP_DB_HOST must be set");
    let db_port: String = env::var("PJSIP_DB_PORT").expect("PJSIP_DB_PORT must be set");
    let db_catalog: String = env::var("PJSIP_DB_CATALOG").expect("PJSIP_DB_CATALOG must be set");
    let db_ssl_mode: String = env::var("PJSIP_DB_SSL_MODE").expect("PJSIP_DB_SSL_MODE must be set");

    // Pool configuration
    let db_max_conn: u32 = env::var("PJSIP_DB_POOL_SIZE")
        .expect("PJSIP_DB_POOL_SIZE must be set")
        .parse()
        .expect("PJSIP_DB_POOL_SIZE must be a valid number");

    let db_max_lifetime: u64 = env::var("PJSIP_DB_MAX_LIFETIME")
        .expect("PJSIP_DB_MAX_LIFETIME must be set")
        .parse()
        .expect("PJSIP_DB_MAX_LIFETIME must be a valid number");

    let db_max_idle: u64 = env::var("PJSIP_DB_MAX_IDLE")
        .expect("PJSIP_DB_MAX_IDLE must be set")
        .parse()
        .expect("PJSIP_DB_MAX_IDLE must be a valid number");

    let db_timeout: u64 = env::var("PJSIP_DB_TIMEOUT")
        .expect("PJSIP_DB_TIMEOUT must be set")
        .parse()
        .expect("PJSIP_DB_TIMEOUT must be a valid number");

    let url: String = format!(
        "{}://{}:{}@{}:{}/{}?sslmode={}",
        db_scheme, db_user, db_password, db_host, db_port, db_catalog, db_ssl_mode
    );

    println!(
        "Connecting to PJSIP database at: {}://{}:***@{}:{}/{}?sslmode={}",
        db_scheme, db_user, db_host, db_port, db_catalog, db_ssl_mode
    );

    PgPoolOptions::new()
        .max_connections(db_max_conn)
        .max_lifetime(Some(std::time::Duration::from_secs(db_max_lifetime)))
        .idle_timeout(Some(std::time::Duration::from_secs(db_max_idle)))
        .acquire_timeout(std::time::Duration::from_secs(db_timeout))
        .connect(&url)
        .await
}

#[tokio::main]
async fn main() {
    // initialize tracing(logging)
    tracing_subscriber::fmt::init();
    // TODO tracing configurations

    // configurations
    dotenv().ok();

    info!("Starting AI Talker API...");
    info!("Loading database configuration...");

    // Debug: print environment variables
    debug!("PJSIP_DB_SCHEME: {:?}", env::var("PJSIP_DB_SCHEME"));
    debug!("PJSIP_DB_USER: {:?}", env::var("PJSIP_DB_USER"));
    debug!("PJSIP_DB_HOST: {:?}", env::var("PJSIP_DB_HOST"));
    debug!("PJSIP_DB_PORT: {:?}", env::var("PJSIP_DB_PORT"));
    debug!("PJSIP_DB_CATALOG: {:?}", env::var("PJSIP_DB_CATALOG"));

    info!("Start database connection pool creation");
    match create_pjsip_pool().await {
        Ok(pool) => {
            info!("Database connection pool created successfully");
            let state = AppState { pjsip_db: pool };

            let router: Router = restapi::routes::root::create_router(state.clone());

            let listen_ipv4: String = env::var("LISTEN_IPV4").expect("LISTEN_IPV4 must be set");
            let listen_port_v4: String =
                env::var("LISTEN_PORT_V4").expect("LISTEN_PORT_V4 must be set");
            let listen_addr: String = format!("{}:{}", listen_ipv4, listen_port_v4);

            tracing::info!("Starting server on {}", listen_addr);
            let listener: TcpListener = tokio::net::TcpListener::bind(&listen_addr).await.unwrap();
            axum::serve(listener, router).await.unwrap();
        }
        Err(e) => {
            tracing::error!("Failed to create database connection pool: {}", e);
            std::process::exit(1);
        }
    }
}

// async fn hello_world() -> impl IntoResponse {
//     let response = "Hello, world!";
//     (StatusCode::OK, Json(response))
// }
