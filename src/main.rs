mod handlers;
mod state;
mod routes;
pub mod utils;
pub mod models;
pub mod auth;

use std::env;
use crate::state::YukinoState;
use sqlx::SqlitePool;
use std::sync::Arc;
use dotenvy::dotenv;
use sha2::{Digest, Sha256};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:yukino.db".to_string());
    let pool = SqlitePool::connect(db_url.as_str()).await.unwrap();

    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable is missing!");
    let mut hasher = Sha256::new();
    hasher.update(telegram_bot_token.as_bytes());

    let state = Arc::new(YukinoState {
        db: pool,
        tg_bot_token: hasher.finalize().into(),
    });

    let app = routes::routers(state);

    let addr = env::var("LISTEN_ADDR").unwrap_or("0.0.0.0:8088".to_string());
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}