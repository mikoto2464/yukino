use crate::auth::Backend;
use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::{delete, post};
use axum_login::login_required;
use std::sync::Arc;

pub fn project_routes() -> Router<Arc<YukinoState>> {
    let need_login_router = Router::new()
        .route("/all", delete(handlers::project::get_projects))
        .route_layer(login_required!(Backend, login_url = "/login"));

    Router::new()
        .merge(need_login_router)
        .route("/", post(handlers::project::create_project))
}
