use crate::auth::Backend;
use crate::models::credential::{AuthCredential, Provider};
use crate::models::user::User;
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::response::{YukinoJson, YukinoResponse};
use crate::utils::telegram_hash::{TelegramCallbackParams, verify_telegram_hash};
use axum::Json;
use axum::extract::State;
use std::sync::Arc;

pub type AuthSession = axum_login::AuthSession<Backend>;

pub async fn telegram_callback(
    mut auth_session: AuthSession,
    State(state): State<Arc<YukinoState>>,
    Json(payload): Json<TelegramCallbackParams>,
) -> Result<YukinoJson<User>, YukinoError> {
    if !verify_telegram_hash(&payload, &state.tg_secret_key)? {
        return Err(YukinoError::AuthenticationError(
            "Failed to verify telegram hash.".to_string(),
        ));
    }

    let auth_credential = AuthCredential {
        id: payload.id.to_string(),
        provider: Provider::Telegram,
        nickname: payload.username,
        avatar_url: payload.photo_url,
    };

    let user = auth_session.authenticate(auth_credential).await?.unwrap();

    if auth_session.login(&user).await.is_err() {
        return Err(YukinoError::AuthenticationError(
            "Login failed with unknown error.".to_string(),
        ));
    }

    Ok(YukinoResponse::success(user))
}
