use crate::utils::error::YukinoError;
use crate::utils::error::YukinoError::ConfigError;
use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use serde::Deserialize;

const AUTH_DATE_MAX_AGE_SECS: i64 = 86400;

#[derive(Deserialize)]
pub struct TelegramCallbackParams {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub photo_url: String,
    pub hash: String,
    pub auth_date: i64,
}

pub fn verify_telegram_hash(
    params: &TelegramCallbackParams,
    secret_key: &[u8],
) -> Result<bool, YukinoError> {
    if Utc::now().timestamp() - params.auth_date > AUTH_DATE_MAX_AGE_SECS {
        return Err(YukinoError::AuthenticationError("Telegram authentication expired.".to_string()));
    }

    let data_check_vec = [
        format!("auth_date={}", params.auth_date),
        format!("first_name={}", params.first_name),
        format!("id={}", params.id),
        format!("last_name={}", params.last_name),
        format!("photo_url={}", params.photo_url),
        format!("username={}", params.username),
    ];
    let data_check_string = data_check_vec.join("\n");

    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key)
        .map_err(|_| ConfigError("Invalid key length.".to_string()))?;

    mac.update(data_check_string.as_bytes());

    let hash_bytes = hex::decode(&params.hash)
        .map_err(|_| YukinoError::AuthenticationError("Invalid hex hash.".to_string()))?;

    Ok(mac.verify_slice(&hash_bytes).is_ok())
}