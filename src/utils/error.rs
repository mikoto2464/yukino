use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YukinoError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Config error: {0}")]
    ConfigError(String),
}

impl IntoResponse for YukinoError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            YukinoError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            YukinoError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            YukinoError::ConfigError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            YukinoError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
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
        match error {
            sqlx::Error::RowNotFound => YukinoError::NotFound("Record not found".to_string()),
            _ => YukinoError::DatabaseError(error.to_string()),
        }
    }
}