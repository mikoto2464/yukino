use axum::Json;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct YukinoResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub message: String,
    pub timestamp: i64,
}

pub type YukinoJson<T> = Json<YukinoResponse<T>>;
pub type YukinoErrorJson = Json<YukinoResponse<()>>;

impl<T> YukinoResponse<T> {
    pub fn success(data: T) -> YukinoJson<T> {
        Json(Self {
            data: Some(data),
            message: "ok".to_string(),
            timestamp: Utc::now().timestamp(),
        })
    }
}

impl YukinoResponse<()> {
    pub fn error(msg: impl Into<String>) -> YukinoErrorJson {
        Json(Self {
            data: None,
            message: msg.into(),
            timestamp: Utc::now().timestamp(),
        })
    }
}
