use crate::response::error::ResponseError;
use crate::service;

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("TemplateError: {0}")]
    TemplateService(#[from] service::template::ServiceError),
    #[error("UserError: {0}")]
    UserService(#[from] service::user::UserServiceError),
}

impl ResponseError for ControllerError {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ControllerError::TemplateService(service_err) => service_err.status_code(),
            ControllerError::UserService(service_err) => service_err.status_code(),
        }
    }

    fn error_code(&self) -> crate::response::error::ErrorCode {
        match self {
            ControllerError::TemplateService(service_err) => service_err.error_code(),
            ControllerError::UserService(service_err) => service_err.error_code(),
        }
    }

    fn user_message(&self) -> String {
        match self {
            ControllerError::TemplateService(service_err) => service_err.user_message(),
            ControllerError::UserService(service_err) => service_err.user_message(),
        }
    }

    fn technical_description(&self) -> Option<String> {
        match self {
            ControllerError::TemplateService(service_err) => service_err.technical_description(),
            ControllerError::UserService(service_err) => service_err.technical_description(),
        }
    }

    fn technical_details(&self) -> Option<String> {
        match self {
            ControllerError::TemplateService(service_err) => service_err.technical_details(),
            ControllerError::UserService(service_err) => service_err.technical_details(),
        }
    }
}
