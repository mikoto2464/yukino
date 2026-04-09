use crate::auth::Backend;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use axum::response::IntoResponse;
use axum::Router;
use axum_login::AuthManagerLayer;
use std::sync::Arc;
use tower_sessions_sqlx_store::SqliteStore;

mod auth;
mod device;
mod project;

async fn handler_404() -> impl IntoResponse {
    YukinoError::NotFound("This path does not exist.".to_string())
}

// TODO:
//     let protected_routes = Router::new()
//         .route("/profile", get(handlers::profile::my_profile))
//         .route("/admin", get(handlers::admin::admin_dashboard))
//         .route_layer(login_required!(Backend, login_url = "/login"));
//     let app = Router::new()
//         .route("/login", get(|| async { "Please trigger OAuth login." }))
//         .route("/oauth/callback", get(auth::handlers::oauth_callback))
//         .nest("/api", protected_routes)
//         .layer(auth_layer)
//         .with_state(state);
pub fn app_routers(
    state: Arc<YukinoState>,
    auth_layer: AuthManagerLayer<Backend, SqliteStore>,
) -> Router {
    Router::new()
        .nest("/auth", auth::auth_routes())
        .nest("/api/v1/device", device::device_routes())
        .nest("/api/v1/project", project::project_routes())
        .fallback(handler_404)
        .layer(auth_layer)
        .with_state(state)
}
