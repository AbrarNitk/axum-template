use crate::response::error::{ErrorCode, ServiceErrorMapping};
use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum SimpleServiceError {
    #[error("Simple error occurred")]
    SimpleError,
    #[error("Another simple error")]
    AnotherError,
}

impl ServiceErrorMapping for SimpleServiceError {
    fn map_to_status_code(&self) -> StatusCode {
        match self {
            SimpleServiceError::SimpleError => StatusCode::BAD_REQUEST,
            SimpleServiceError::AnotherError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn map_to_error_code(&self) -> ErrorCode {
        match self {
            SimpleServiceError::SimpleError => ErrorCode::BadRequest,
            SimpleServiceError::AnotherError => ErrorCode::InternalServerError,
        }
    }

    // Note: We don't override technical_description or technical_details
    // So it will fall back to ResponseError::error_description and ResponseError::error_details
}
