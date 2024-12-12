use actix_web::{HttpResponse, ResponseError};
use aws_sdk_s3::presigning::PresigningConfigError;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Presigning error: {0}")]
    PresigningError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(json!({
                "error": msg
            })),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(json!({
                "error": msg
            })),
            AppError::PresigningError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": msg
            })),
        }
    }
}

impl From<PresigningConfigError> for AppError {
    fn from(error: PresigningConfigError) -> Self {
        AppError::PresigningError(error.to_string())
    }
}
