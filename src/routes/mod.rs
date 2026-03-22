use std::sync::Arc;
use axum::response::IntoResponse;
use axum::Router;
use crate::error::YukinoError;
use crate::state::YukinoState;

mod device;

async fn handler_404() -> impl IntoResponse {
    YukinoError::NotFound("该接口路径不存在".to_string())
}

pub fn routers(state: Arc<YukinoState>) -> Router {
    Router::new()
        .nest("/api/v1/device", device::device_routes())
        .fallback(handler_404)
        .with_state(state)
}