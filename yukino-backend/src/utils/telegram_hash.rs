use crate::utils::error::YukinoError;
use crate::utils::error::YukinoError::ConfigError;
use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::collections::BTreeMap;

const AUTH_DATE_MAX_AGE_SECS: i64 = 86400;

pub fn verify_telegram_hash(
    params: &BTreeMap<String, String>,
    secret_key: &[u8],
) -> Result<bool, YukinoError> {
    let Some(hash_hex) = params.get("hash") else {
        return Ok(false);
    };

    let Some(auth_date_str) = params.get("auth_date") else {
        return Ok(false);
    };
    let auth_date: i64 = auth_date_str.parse().map_err(|_| {
        YukinoError::AuthenticationError("Invalid auth_date.".to_string())
    })?;
    if Utc::now().timestamp() - auth_date > AUTH_DATE_MAX_AGE_SECS {
        return Err(YukinoError::AuthenticationError(
            "Telegram auth_date expired.".to_string(),
        ));
    }

    let hash_bytes = match hex::decode(hash_hex) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(false),
    };

    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key)
        .map_err(|_| ConfigError("Invalid key length.".to_string()))?;

    let mut is_first = true;
    for (k, v) in params.iter() {
        if k == "hash" {
            continue;
        }

        if !is_first {
            mac.update(b"\n");
        }
        mac.update(k.as_bytes());
        mac.update(b"=");
        mac.update(v.as_bytes());

        is_first = false;
    }

    Ok(mac.verify_slice(&hash_bytes).is_ok())
}
