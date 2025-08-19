use crate::controller::types::templates::{CreateReq, CreateRes, GetResponse};
use crate::response::error::{ErrorCode, ResponseError};
use axum::http::StatusCode;

pub async fn get(id: String) -> Result<CreateRes, ServiceError> {
    Ok(CreateRes { id: id })
}

pub async fn create(req: CreateReq) -> Result<GetResponse, ServiceError> {
    Ok(GetResponse {
        id: "1".to_string(),
        name: req.name,
        description: req.description,
        content: req.content,
    })
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Template not found with ID: {0}")] // Technical message with ID
    NotFound(String),
    #[error("Internal server error occurred")] // Simple enough for users
    InternalServerError,
    #[error("Invalid request data")] // Simple enough for users
    BadRequest,
    #[error("Authentication required")] // Simple enough for users
    UnAuthorized,
}

impl ResponseError for ServiceError {
    // Only implement error_code - status_code is automatically derived
    fn error_code(&self) -> ErrorCode {
        match self {
            ServiceError::NotFound(_) => ErrorCode::NotFound,
            ServiceError::InternalServerError => ErrorCode::InternalServerError,
            ServiceError::BadRequest => ErrorCode::BadRequest,
            ServiceError::UnAuthorized => ErrorCode::UnAuthorized,
        }
    }

    // Override user_message only when thiserror message is not user-friendly
    fn user_message(&self) -> String {
        match self {
            ServiceError::NotFound(_) => "The requested template could not be found".to_string(),
            // For simple cases, use thiserror message
            ServiceError::InternalServerError => self.to_string(), // "Internal server error occurred"
            ServiceError::BadRequest => self.to_string(),          // "Invalid request data"
            ServiceError::UnAuthorized => self.to_string(),        // "Authentication required"
        }
    }

    // Provide specific technical context about what happened
    fn technical_description(&self) -> Option<String> {
        match self {
            ServiceError::NotFound(id) => Some(format!(
                "Template with ID '{}' was not found in the database",
                id
            )),
            ServiceError::InternalServerError => {
                Some("Database connection failed or service unavailable".to_string())
            }
            ServiceError::BadRequest => Some(
                "Request validation failed - missing required fields or invalid format".to_string(),
            ),
            ServiceError::UnAuthorized => {
                Some("JWT token missing, expired, or invalid".to_string())
            }
        }
    }

    // Provide full technical details (optional, controlled by backend)
    fn technical_details(&self) -> Option<String> {
        match self {
            ServiceError::NotFound(id) => Some(format!("Template lookup failed for ID: {}. Database query returned no results. This could indicate the template was deleted or the ID is incorrect.", id)),
            ServiceError::InternalServerError => Some("Database connection pool exhausted. Connection timeout after 30 seconds. Check database server status and connection pool configuration.".to_string()),
            ServiceError::BadRequest => Some("Request body validation failed. Required fields: name (string, 1-100 chars), content (string, non-empty). Received: name='', content=''.".to_string()),
            ServiceError::UnAuthorized => Some("JWT token validation failed. Token expired at 2024-01-15T10:30:00Z. Current time: 2024-01-15T11:00:00Z. Token signature verification failed.".to_string()),
        }
    }
}
