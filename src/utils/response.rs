use axum::Json;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct YukinoResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: String,
    pub timestamp: i64,
}

pub type YukinoJson<T> = Json<YukinoResponse<T>>;

impl<T> YukinoResponse<T> {
    pub fn success(data: T) -> YukinoJson<T> {
        Json(Self {
            success: true,
            data,
            message: "ok".to_string(),
            timestamp: Utc::now().timestamp(),
        })
    }
}
