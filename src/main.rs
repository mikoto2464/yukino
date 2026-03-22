mod error;
mod handlers;
mod state;
mod routes;

use std::sync::Arc;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use crate::state::YukinoState;

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