use crate::models::user::{Role, User};
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use axum::extract::{Query, State};
use std::collections::BTreeMap;
use std::sync::Arc;
use crate::utils::telegram_hash::verify_telegram_hash;

async fn oauth_callback_telegram(
    State(state): State<Arc<YukinoState>>,
    Query(params): Query<BTreeMap<String, String>>
) -> Result<YukinoJson<User>, YukinoError> {
    if !verify_telegram_hash(&params, &state.tg_bot_token)? {
        Err(YukinoError::AuthenticationError("Failed to verify telegram hash".to_string()))?
    }

    Ok(YukinoResponse::success(User {
        id: 123,
        nickname: "test".to_string(),
        avatar_url: "test".to_string(),
        role: Role::User,
        auth_stamp: "test".to_string(),
    }))
}