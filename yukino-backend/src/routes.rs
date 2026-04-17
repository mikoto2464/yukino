use crate::auth::Backend;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use axum::Router;
use axum::response::IntoResponse;
use axum_login::AuthManagerLayer;
use std::sync::Arc;
use tower_sessions_sqlx_store::SqliteStore;

mod auth;
pub mod device;
pub mod user;

async fn handler_404() -> impl IntoResponse {
    YukinoError::NotFound("This path does not exist.".to_string())
}

pub fn app_routers(
    state: Arc<YukinoState>,
    auth_layer: AuthManagerLayer<Backend, SqliteStore>,
) -> Router {
    Router::new()
        .nest("/api", api_routes())
        .fallback(handler_404)
        .layer(auth_layer)
        .with_state(state)
}

pub fn api_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .nest("/auth", auth::auth_routes())
        .nest("/user", user::user_routes())
        .nest("/device", device::device_routes())
}
