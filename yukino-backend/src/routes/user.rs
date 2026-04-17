use crate::handlers;
use crate::state::YukinoState;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;
use axum_login::login_required;
use crate::auth::Backend;

pub fn user_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/session", delete(handlers::user::logout))
        .route("/me", get(handlers::user::me))
        .route("/devices", get(handlers::user::devices).post(handlers::user::create_device))
        .route("/devices/{hardware_id}", delete(handlers::user::delete_device))
        .route_layer(login_required!(Backend, login_url = "/login"))
}
