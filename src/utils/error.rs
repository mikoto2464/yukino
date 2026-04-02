use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use chrono::Utc;
use serde_json::json;

pub enum YukinoError {
    NotFound(String),
    DatabaseError(String),
}

impl IntoResponse for YukinoError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            YukinoError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            YukinoError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "success": false,
            "data": "",
            "message": error_message,
            "timestamp": Utc::now().timestamp()
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for YukinoError {
    fn from(error: sqlx::Error) -> Self {
        YukinoError::DatabaseError(error.to_string())
    }
}