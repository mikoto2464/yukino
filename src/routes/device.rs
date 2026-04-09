use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn device_routes() -> Router<Arc<YukinoState>> {
    Router::new().route("/{user_id}", get(handlers::device::get_devices))
}
