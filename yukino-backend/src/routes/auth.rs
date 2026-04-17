use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn auth_routes() -> Router<Arc<YukinoState>> {
    Router::new().route(
        "/telegram/callback",
        post(handlers::auth::telegram_callback),
    )
}
