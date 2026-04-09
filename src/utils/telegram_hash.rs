use hmac::{Hmac, Mac, KeyInit};
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::utils::error::YukinoError;
use crate::utils::error::YukinoError::ConfigError;

pub fn verify_telegram_hash(
    params: &BTreeMap<String, String>,
    secret_key: &[u8]
) -> Result<bool, YukinoError> {

    let Some(hash_hex) = params.get("hash") else {
        return Ok(false);
    };

    let hash_bytes = match hex::decode(hash_hex) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(false),
    };

    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key)
        .map_err(|_| ConfigError("Invalid key length".to_string()))?;

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