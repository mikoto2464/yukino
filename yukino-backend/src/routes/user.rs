use crate::auth::Backend;
use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::{get};
use axum_login::login_required;
use std::sync::Arc;

pub fn user_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/me", get(handlers::user::me))
        .route_layer(login_required!(Backend, login_url = "/login"))
}
