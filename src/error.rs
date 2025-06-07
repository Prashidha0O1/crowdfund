use axum::{http::StatusCode, response::{IntoResponse, Response, Json}};
use serde_json::json;
use thiserror::Error;

/// A common error type for the entire application to handle various error sources.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] mysql_async::Error),

    #[error("Environment variable not found: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("HTTP request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Template rendering error: {0}")]
    TeraError(#[from] tera::Error),

    #[error("User not found")]
    UserNotFound,

    #[error("Authentication error: {0}")]
    AuthError(String),
    
    // The unused 'InvalidState' variant has been removed.
}

/// Allows AppError to be converted into an HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        log::error!("Error: {:?}", self);
        
        let (status, error_message) = match self {
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("A database error occurred: {}", e)),
            AppError::EnvVarError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Server configuration error: {}", e)),
            AppError::ReqwestError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("An error occurred with an external service: {}", e)),
            AppError::TeraError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to render the page: {}", e)),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "The requested user was not found.".to_string()),
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg),
        };
        
        // Return a structured JSON response
        (status, Json(json!({"error": error_message}))).into_response()
    }
}
