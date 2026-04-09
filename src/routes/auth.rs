use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn auth_routes() -> Router<Arc<YukinoState>> {
    Router::new().route("/telegram", get(handlers::auth::telegram_callback))
}
