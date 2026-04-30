use crate::auth::Backend;
use crate::utils::response::YukinoResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_login::Error as AxumLoginError;
use thiserror::Error;
use tracing::{error, warn};

#[derive(Error, Debug)]
pub enum YukinoError {
    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Invalid parameters: {0}")]
    InvalidParametersError(String),

    #[error("Configuration error occurred")]
    ConfigError(String),

    #[error("Database error occurred")]
    DatabaseError(#[from] sqlx::Error),
}

impl YukinoError {
    fn status_code(&self) -> StatusCode {
        match self {
            YukinoError::NotFound(_) => StatusCode::NOT_FOUND,
            YukinoError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            YukinoError::DatabaseError(_) | YukinoError::ConfigError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            YukinoError::InvalidParametersError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for YukinoError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        if status.is_server_error() {
            error!("Server Error: {:?}", self);
        } else {
            warn!("Client Error: {}", self);
        }

        let user_message = if status.is_server_error() {
            "Internal server error. Please try again later.".to_string()
        } else {
            self.to_string()
        };

        let body = YukinoResponse::error(user_message);

        (status, body).into_response()
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
