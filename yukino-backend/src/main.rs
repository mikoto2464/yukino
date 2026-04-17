pub mod auth;
mod handlers;
pub mod models;
mod routes;
mod state;
pub mod utils;

use crate::auth::Backend;
use crate::state::YukinoState;
use axum_login::AuthManagerLayerBuilder;
use axum_login::tower_sessions::{ExpiredDeletion, SessionManagerLayer};
use dotenvy::dotenv;
use sha2::{Digest, Sha256};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_sessions_sqlx_store::SqliteStore;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    // DATABASE_URL
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:yukino.db".to_string());
    let options = SqliteConnectOptions::from_str(&db_url)
        .unwrap()
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(5));
    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .expect("Failed to connect to the database");

    // TELEGRAM_BOT_TOKEN
    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable is missing.");
    let mut hasher = Sha256::new();
    hasher.update(telegram_bot_token.as_bytes());

    let state = Arc::new(YukinoState {
        tg_secret_key: hasher.finalize().into(),
        db: pool.clone(),
    });

    let session_store = SqliteStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("Failed to migrate the database");

    // session cleaner
    let store_clone = session_store.clone();
    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(3600)).await;
            if let Err(e) = store_clone.delete_expired().await {
                error!("Failed to clean expired sessions: {:?}", e);
            }
        }
    });

    // auth layer
    let backend = Backend { db: pool.clone() };
    let session_layer = SessionManagerLayer::new(session_store);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = routes::app_routers(state, auth_layer);
    let addr = env::var("LISTEN_ADDR").unwrap_or("0.0.0.0:8088".to_string());
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    info!(
        "Server running on http://{}",
        listener
            .local_addr()
            .expect("Failed to bind listener socket")
    );

    axum::serve(listener, app)
        .await
        .expect("Failed to run server");
}
