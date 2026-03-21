mod error;
mod services;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));
    let listener = TcpListener::bind("0.0.0.0:8088").await.unwrap();
    println!("Server running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}