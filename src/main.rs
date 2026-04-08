mod handlers;
mod state;
mod routes;
pub mod utils;
pub mod models;
pub mod auth;

use crate::state::YukinoState;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite:yukino.db").await.unwrap();

    let state = Arc::new(YukinoState {
        db: pool,
    });

    let app = routes::routers(state);
    let listener = TcpListener::bind("0.0.0.0:8088").await.unwrap();
    println!("Server running on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}