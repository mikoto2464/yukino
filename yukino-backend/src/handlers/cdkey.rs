use std::sync::Arc;
use axum::extract::{Path, State};
use crate::handlers::auth::AuthSession;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use crate::models::cdkey::{Cdkey, Period};
use crate::utils::error::YukinoError::InvalidParamentsError;

pub async fn activate_cdkey(
    State(state): State<Arc<YukinoState>>,
    auth_session: AuthSession,
    Path(cdkey): Path<String>,
) -> Result<YukinoJson<String>, YukinoError> {
    let cdkeys = sqlx::query_as!(
        Cdkey,
        r#"
        select cdkey, project_id, period as 'period: Period'
        from cdkeys
        where cdkey = ?
        "#,
        cdkey
    )
        .fetch_all(&state.db)
        .await?;

    if cdkey.is_empty() {
        return Err(InvalidParamentsError("The CD key is invalid.".to_string()));
    }

    sqlx::query!(
        r#"
        delete from cdkeys
        where cdkey = ?
        "#,
        cdkey
    )
        .execute(&state.db)
        .await?;

    Ok(YukinoResponse::success(cdkey))
}