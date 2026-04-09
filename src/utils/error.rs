use std::error::Error;
use std::fmt;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use chrono::Utc;
use serde_json::json;

#[derive(Debug)]
pub enum YukinoError {
    NotFound(String),
    DatabaseError(String),
}

impl fmt::Display for YukinoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YukinoError::DatabaseError(msg) => msg.fmt(f),
            YukinoError::NotFound(msg) => msg.fmt(f),
        }
    }
}

impl Error for YukinoError {}

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
        match error {
            sqlx::Error::RowNotFound => YukinoError::NotFound("Record not found".to_string()),
            _ => YukinoError::DatabaseError(error.to_string())
        }
    }
}