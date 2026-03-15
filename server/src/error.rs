use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Config(envy::Error),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("http client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("github rate limited until {reset_at}")]
    GitHubRateLimited { reset_at: i64 },
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("internal worker queue is unavailable")]
    QueueUnavailable,
    #[error("unexpected error: {0}")]
    Other(String),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::GitHubRateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
            AppError::QueueUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Config(_)
            | AppError::Database(_)
            | AppError::Migration(_)
            | AppError::Http(_)
            | AppError::Serialization(_)
            | AppError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorBody {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
