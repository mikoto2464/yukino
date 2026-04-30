use crate::handlers::auth::AuthSession;
use crate::models::user::Role;
use crate::models::user::User;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use axum::extract::State;
use std::sync::Arc;

pub async fn me(
    State(_state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
) -> Result<YukinoJson<User>, YukinoError> {
    let user = auth_session.user.unwrap();

    Ok(YukinoResponse::success(user))
}
