use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::{delete, post};
use std::sync::Arc;

pub fn auth_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/session/telegram", post(handlers::auth::telegram_callback))
        .route("/session", delete(handlers::auth::logout))
}
