use crate::utils::error::YukinoError;
use crate::state::YukinoState;
use axum::response::IntoResponse;
use axum::Router;
use std::sync::Arc;

mod device;
mod project;

async fn handler_404() -> impl IntoResponse {
    YukinoError::NotFound("该接口路径不存在".to_string())
}

pub fn routers(state: Arc<YukinoState>) -> Router {
    Router::new()
        .nest("/api/v1/device", device::device_routes())
        .nest("/api/v1/project", project::project_routes())
        .fallback(handler_404)
        .with_state(state)
}