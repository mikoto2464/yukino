use crate::handlers::auth::AuthSession;
use crate::models::user::Role;
use crate::models::user::User;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use axum::extract::State;
use std::sync::Arc;

pub async fn me(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
) -> Result<YukinoJson<User>, YukinoError> {
    let user = auth_session.user.unwrap();

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, nickname, avatar_url, role AS 'role: Role', auth_stamp
        FROM users
        WHERE id = ?
        "#,
        user.id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(YukinoResponse::success(user))
}
