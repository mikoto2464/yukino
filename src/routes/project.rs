use crate::services;
use crate::state::YukinoState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn project_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/create/{name}", get(services::project::create_project))
}