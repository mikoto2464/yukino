use crate::handlers;
use crate::state::YukinoState;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

pub fn auth_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/telegram/callback", post(handlers::auth::telegram_callback))
}
