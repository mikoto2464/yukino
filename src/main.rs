use axum::{Router, routing::get};
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let launcher_listener = TcpListener::bind("0.0.0.0:8088").await.unwrap();
    axum::serve(launcher_listener, app).await.unwrap();
}