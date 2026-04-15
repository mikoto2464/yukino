
use crate::handlers::auth::AuthSession;
use crate::models::user::Role;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use axum::extract::State;
use std::sync::Arc;
use crate::models::user::User;

pub async fn me(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession
) -> Result<YukinoJson<User>, YukinoError> {
    let user = auth_session.user.unwrap();

    let user = sqlx::query_as!(
        User,
        r#"
        select id , nickname, avatar_url, role as 'role: Role', auth_stamp
        from users
        where id = ?
        "#,
        user.id
    )
        .fetch_one(&state.db)
        .await?;

    Ok(YukinoResponse::success(user))
}
