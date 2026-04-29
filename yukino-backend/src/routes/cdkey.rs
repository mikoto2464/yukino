use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn cdkey_routes() -> Router<Arc<YukinoState>> {
    Router::new().route(
        "/",
        post(handlers::cdkey::activate_cdkey),
    )
}
