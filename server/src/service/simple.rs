use crate::response::error::{ErrorCode, ResponseError};
use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum SimpleServiceError {
    #[error("Simple error occurred")]
    SimpleError,
    #[error("Another simple error")]
    AnotherError,
}

impl ResponseError for SimpleServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            SimpleServiceError::SimpleError => StatusCode::BAD_REQUEST,
            SimpleServiceError::AnotherError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> ErrorCode {
        match self {
            SimpleServiceError::SimpleError => ErrorCode::BadRequest,
            SimpleServiceError::AnotherError => ErrorCode::InternalServerError,
        }
    }

    // Note: We don't override user_message, technical_description, or technical_details
    // So it will use the default implementations from ResponseError
}
