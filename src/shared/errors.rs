use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalError(msg) => {
                tracing::error!("Internal server error: {:?}", msg);

                (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected internal error occurred.".to_string())
            },
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Internal(err) => {
                tracing::error!("Internal server error: {:?}", err);

                (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected internal error occurred.".to_string())
            }
        };

        let body = Json(json!({
            "status": status.as_u16(),
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
