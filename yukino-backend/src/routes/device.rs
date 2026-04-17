use crate::auth::Backend;
use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::{delete, get};
use axum_login::login_required;
use std::sync::Arc;

pub fn device_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route(
            "/",
            get(handlers::device::get_devices).post(handlers::device::create_device),
        )
        .route("/{hardware_id}", delete(handlers::device::delete_device))
        .route_layer(login_required!(Backend, login_url = "/login"))
}
