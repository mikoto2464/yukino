use serde::Serialize;
use axum::Json;
use chrono::Utc;

#[derive(Serialize)]
pub struct YukinoResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: i64,
}

pub type YukinoJson<T> = Json<YukinoResponse<T>>;

impl<T> YukinoResponse<T> {
    pub fn success(data: T) -> YukinoJson<T> {
        Json(Self {
            success: true,
            data: Some(data),
            message: "ok".to_string(),
            timestamp: Utc::now().timestamp(),
        })
    }
}