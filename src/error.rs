use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;

pub enum YukinoError {
    BadRequest(String),
    NotFound(String),
    DatabaseError(String),
}

impl IntoResponse for YukinoError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            YukinoError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            YukinoError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            YukinoError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": true,
            "msg": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for YukinoError {
    fn from(error: sqlx::Error) -> Self {
        YukinoError::DatabaseError(error.to_string())
    }
}