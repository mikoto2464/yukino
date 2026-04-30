use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;
use axum_login::login_required;
use crate::auth::Backend;

pub fn subscription_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/", post(handlers::subscription::redemption))
        .route_layer(login_required!(Backend, login_url = "/login"))
}
