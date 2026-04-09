use crate::auth::Backend;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_login::Error as AxumLoginError;
use chrono::Utc;
use serde_json::Value::Null;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YukinoError {
    #[error("ResourceNotFound::{0}")]
    NotFound(String),

    #[error("Database::{0}")]
    DatabaseError(String),

    #[error("Authentication::{0}")]
    AuthenticationError(String),

    #[error("Config::{0}")]
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
            "data": Null,
            "message": error_message,
            "timestamp": Utc::now().timestamp()
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for YukinoError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => {
                YukinoError::NotFound("Record not found.".to_string())
            }
            _ => YukinoError::DatabaseError(error.to_string()),
        }
    }
}

impl From<AxumLoginError<Backend>> for YukinoError {
    fn from(err: AxumLoginError<Backend>) -> Self {
        match err {
            AxumLoginError::Backend(e) => e,
            AxumLoginError::Session(e) => {
                YukinoError::AuthenticationError(format!("Session error: {}", e))
            }
        }
    }
}
