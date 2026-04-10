use crate::handlers;
use crate::state::YukinoState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use axum_login::login_required;
use crate::auth::Backend;

pub fn device_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/get", get(handlers::device::get_devices))
        .route_layer(login_required!(Backend, login_url = "/login"))
}
