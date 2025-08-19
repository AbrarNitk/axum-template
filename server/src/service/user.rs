use crate::response::error::{ErrorCode, ResponseError};
use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum UserServiceError {
    #[error("User not found with ID: {0}")] // Technical message with ID
    UserNotFound(String),
    #[error("Invalid email format: {0}")] // Technical message with details
    InvalidEmail(String),
    #[error("User already exists")] // Simple enough for users
    UserAlreadyExists,
    #[error("Database connection failed: {0}")] // Technical message with error
    DatabaseError(String),
}

impl ResponseError for UserServiceError {
    // Only implement error_code - status_code is automatically derived
    fn error_code(&self) -> ErrorCode {
        match self {
            UserServiceError::UserNotFound(_) => ErrorCode::NotFound,
            UserServiceError::InvalidEmail(_) => ErrorCode::BadRequest,
            UserServiceError::UserAlreadyExists => ErrorCode::BadRequest, // or we could add Conflict to ErrorCode
            UserServiceError::DatabaseError(_) => ErrorCode::InternalServerError,
        }
    }

    // Override user_message only when thiserror message is not user-friendly
    fn user_message(&self) -> String {
        match self {
            UserServiceError::UserNotFound(_) => {
                "The requested user could not be found".to_string()
            }
            UserServiceError::InvalidEmail(_) => "Please provide a valid email address".to_string(),
            UserServiceError::UserAlreadyExists => self.to_string(), // Will use: "User already exists"
            UserServiceError::DatabaseError(_) => {
                "Unable to process your request at this time".to_string()
            }
        }
    }

    // Provide specific technical context about what happened
    fn technical_description(&self) -> Option<String> {
        match self {
            UserServiceError::UserNotFound(id) => Some(format!(
                "User with ID '{}' was not found in the database",
                id
            )),
            UserServiceError::InvalidEmail(email) => {
                Some(format!("Email '{}' does not match required format", email))
            }
            UserServiceError::UserAlreadyExists => {
                Some("User creation failed - email address already registered".to_string())
            }
            UserServiceError::DatabaseError(reason) => {
                Some(format!("Database operation failed: {}", reason))
            }
        }
    }

    // Provide full technical details (optional, controlled by backend)
    fn technical_details(&self) -> Option<String> {
        match self {
            UserServiceError::UserNotFound(id) => Some(format!("User lookup failed for ID: {}. Database query 'SELECT * FROM users WHERE id = ?' returned no results. This could indicate the user was deleted, the ID is incorrect, or there's a data consistency issue.", id)),
            UserServiceError::InvalidEmail(email) => Some(format!("Email validation failed for: {}. Expected format: user@domain.com. Received: {}. Validation regex: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{{2,}}$", email, email)),
            UserServiceError::UserAlreadyExists => Some("User creation failed due to unique constraint violation. Email address 'john@example.com' is already registered to user ID 'user_456'. Check if user is trying to create duplicate account or if there's a data migration issue.".to_string()),
            UserServiceError::DatabaseError(reason) => Some(format!("Database operation failed with error: {}. Connection pool status: 5/10 connections active. Last successful query: 2 minutes ago. Check database server logs for more details.", reason)),
        }
    }
}

// Mock response types
#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
}

// Mock service functions
pub async fn get_user(id: String) -> Result<User, UserServiceError> {
    // Mock implementation - would call actual database/service
    if id == "not_found" {
        return Err(UserServiceError::UserNotFound(id));
    }
    Ok(User {
        id,
        email: "user@example.com".to_string(),
        name: "John Doe".to_string(),
    })
}

pub async fn create_user(payload: CreateUserReq) -> Result<User, UserServiceError> {
    // Mock implementation - would call actual database/service
    if payload.email.contains("invalid") {
        return Err(UserServiceError::InvalidEmail(payload.email.clone()));
    }
    if payload.email.contains("exists") {
        return Err(UserServiceError::UserAlreadyExists);
    }
    if payload.email.contains("db_error") {
        return Err(UserServiceError::DatabaseError(
            "Connection timeout".to_string(),
        ));
    }
    Ok(User {
        id: "new_user_id".to_string(),
        email: payload.email,
        name: payload.name,
    })
}

// Mock request type
#[derive(Clone)]
pub struct CreateUserReq {
    pub email: String,
    pub name: String,
}
