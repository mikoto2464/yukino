use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::state::YukinoState;
use crate::handlers;

pub fn device_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/{user_id}", get(handlers::device::get_devices))
}