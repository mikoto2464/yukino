use crate::handlers;
use crate::state::YukinoState;
use axum::Router;
use axum::routing::{delete, post};
use std::sync::Arc;

pub fn admin_routes() -> Router<Arc<YukinoState>> {
    Router::new()
        .route("/cdkeys", post(handlers::auth::telegram_callback))
}
