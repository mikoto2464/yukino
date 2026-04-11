use crate::auth::Backend;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use axum::response::IntoResponse;
use axum::Router;
use axum_login::{login_required, AuthManagerLayer};
use std::sync::Arc;
use axum::routing::get;
use tower_sessions_sqlx_store::SqliteStore;
use crate::handlers;

mod auth;
mod device;
mod project;

async fn handler_404() -> impl IntoResponse {
    YukinoError::NotFound("This path does not exist.".to_string())
}

pub fn app_routers(
    state: Arc<YukinoState>,
    auth_layer: AuthManagerLayer<Backend, SqliteStore>,
) -> Router {
    Router::new()
        .route("/login", get(|| async { "Please trigger OAuth login." }))
        .nest("/auth", auth::auth_routes())
        .nest("/api", api_routes())
        .nest("/api/device", device::device_routes())
        .nest("/api/project", project::project_routes())
        .fallback(handler_404)
        .layer(auth_layer)
        .with_state(state)
}

pub fn api_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/devices", get(handlers::device::get_devices))
        .route_layer(login_required!(Backend, login_url = "/login"))
}
