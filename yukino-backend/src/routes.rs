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
pub mod project;
pub mod subscription;
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
        // POST /api/auth/session/telegram 接收telegram返回的auth信息并登录
        // Payload
        // id: i64,
        // first_name: String
        // last_name: String
        // username: String
        // photo_url: String
        // hash: String
        // auth_date: i64
        //
        // DELETE /api/auth/session (LOGIN_REQUIRED)关闭会话，退出登录
        //
        // TODO POST /api/auth/session/qq 接收qq返回的auth信息并登录
        .nest("/auth", auth::auth_routes())
        // GET /api/user/me (LOGIN_REQUIRED)获取用户基本信息
        .nest("/user", user::user_routes())
        // GET /api/devices (LOGIN_REQUIRED)获取用户已绑定的所有设备
        //
        // POST /api/devices (LOGIN_REQUIRED)创建设备
        // Payload
        // hardware_id: String,
        // name: String,
        //
        // DELETE /api/devices/{hardware_id} (LOGIN_REQUIRED)删除用户绑定的设备
        .nest("/devices", device::device_routes())
        // POST /api/projects 创建项目(前端无需调用)
        // Payload
        // create_project_key: String,
        // name: String,
        //
        // GET /api/projects (LOGIN_REQUIRED)获取用户已订阅的所有项目
        .nest("/projects", project::project_routes())
        // POST /api/subscription/redemption 激活订阅
        // Payload
        // cdkey: String
        .nest("/subscriptions", subscription::subscription_routes())
}
