use crate::controller::types::templates::{CreateReq, CreateRes, GetResponse};
use crate::response::error::{ErrorCode, ServiceErrorMapping};
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
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    UnAuthorized,
}

impl ServiceErrorMapping for ServiceError {
    fn map_to_status_code(&self) -> StatusCode {
        match self {
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::UnAuthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn map_to_error_code(&self) -> ErrorCode {
        match self {
            ServiceError::NotFound => ErrorCode::NotFound,
            ServiceError::InternalServerError => ErrorCode::InternalServerError,
            ServiceError::BadRequest => ErrorCode::BadRequest,
            ServiceError::UnAuthorized => ErrorCode::UnAuthorized,
        }
    }
}
