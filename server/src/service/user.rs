use crate::response::error::{ErrorCode, ServiceErrorMapping};
use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum UserServiceError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Database connection failed")]
    DatabaseError,
}

impl ServiceErrorMapping for UserServiceError {
    fn map_to_status_code(&self) -> StatusCode {
        match self {
            UserServiceError::UserNotFound => StatusCode::NOT_FOUND,
            UserServiceError::InvalidEmail => StatusCode::BAD_REQUEST,
            UserServiceError::UserAlreadyExists => StatusCode::CONFLICT,
            UserServiceError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn map_to_error_code(&self) -> ErrorCode {
        match self {
            UserServiceError::UserNotFound => ErrorCode::NotFound,
            UserServiceError::InvalidEmail => ErrorCode::BadRequest,
            UserServiceError::UserAlreadyExists => ErrorCode::BadRequest, // or we could add Conflict to ErrorCode
            UserServiceError::DatabaseError => ErrorCode::InternalServerError,
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
        return Err(UserServiceError::UserNotFound);
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
        return Err(UserServiceError::InvalidEmail);
    }
    if payload.email.contains("exists") {
        return Err(UserServiceError::UserAlreadyExists);
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
