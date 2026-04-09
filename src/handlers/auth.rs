use crate::auth::Backend;
use crate::models::credential::{AuthCredential, Provider};
use crate::state::YukinoState;
use crate::utils::error::YukinoError;
use crate::utils::telegram_hash::verify_telegram_hash;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use std::collections::BTreeMap;
use std::sync::Arc;

pub type AuthSession = axum_login::AuthSession<Backend>;

pub async fn telegram_callback(
    mut auth_session: AuthSession,
    State(state): State<Arc<YukinoState>>,
    Query(params): Query<BTreeMap<String, String>>,
) -> Result<impl IntoResponse, YukinoError> {
    if !verify_telegram_hash(&params, &state.tg_bot_token)? {
        return Err(YukinoError::AuthenticationError(
            "Failed to verify telegram hash.".to_string(),
        ));
    }

    let auth_credential = AuthCredential {
        id: params.get("id").unwrap().to_string(),
        provider: Provider::Telegram,
    };

    let user = auth_session.authenticate(auth_credential).await?;

    match user {
        None => Ok(Redirect::to("/register").into_response()),
        Some(user) => {
            if auth_session.login(&user).await.is_err() {
                return Ok(Redirect::to("/login?error=session_failed").into_response());
            }
            Ok(Redirect::to("/profile").into_response())
        }
    }
}
