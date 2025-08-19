use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(serde::Serialize)]
pub enum ErrorCode {
    NotFound,
    InternalServerError,
    BadRequest,
    UnAuthorized,
}

#[derive(serde::Serialize)]
pub struct ApiError {
    trace_id: String,
    // Note: it can be epoch but it does not make sense because people need to read this
    timestamp: chrono::DateTime<chrono::Utc>,
    code: ErrorCode,
    #[serde(skip)]
    status: StatusCode,

    // User-friendly, actionable message
    message: String,

    // Specific technical context about what happened
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    // Full technical details with backtrace (optional, controlled by backend)
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let error_response = super::ApiErrorResponse {
            success: false,
            error: self,
        };
        (
            status,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            axum::Json(error_response),
        )
            .into_response()
    }
}

pub trait ResponseError: std::fmt::Debug + std::fmt::Display + std::error::Error {
    fn status_code(&self) -> StatusCode;
    fn error_code(&self) -> ErrorCode;

    fn message(&self) -> String {
        self.to_string()
    }

    // Specific technical context about what happened
    fn error_description(&self) -> Option<String> {
        None
    }

    // Full technical details with backtrace (optional, controlled by backend)
    fn error_details(&self) -> Option<String> {
        let mut backtrace = vec![];
        let mut error: &dyn std::error::Error = &self;
        while let Some(source) = error.source() {
            backtrace.push(source);
            error = source;
        }
        Some(
            backtrace
                .into_iter()
                .map(|err| err.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

/// Helper trait to make it easier for service errors to implement ResponseError
/// by providing a mapping from service error variants to HTTP status codes and error codes
pub trait ServiceErrorMapping {
    fn map_to_status_code(&self) -> StatusCode;
    fn map_to_error_code(&self) -> ErrorCode;

    // Optional user-friendly message override
    // Return Some(message) to override thiserror message, None to use thiserror message
    fn user_message(&self) -> Option<String> {
        None
    }

    // Optional specific technical context
    fn technical_description(&self) -> Option<String> {
        None
    }

    // Optional full technical details (overrides default backtrace)
    fn technical_details(&self) -> Option<String> {
        None
    }
}

/// Default implementation for ResponseError when a service error implements ServiceErrorMapping
impl<T> ResponseError for T
where
    T: std::fmt::Debug + std::fmt::Display + std::error::Error + ServiceErrorMapping,
{
    fn status_code(&self) -> StatusCode {
        self.map_to_status_code()
    }

    fn error_code(&self) -> ErrorCode {
        self.map_to_error_code()
    }

    fn error_description(&self) -> Option<String> {
        // Use service's technical_description if provided, otherwise fall back to ResponseError default
        self.technical_description().or_else(|| {
            // Call the default ResponseError implementation
            ResponseError::error_description(self)
        })
    }

    fn error_details(&self) -> Option<String> {
        // Use service's technical_details if provided, otherwise fall back to ResponseError default
        self.technical_details().or_else(|| {
            // Call the default ResponseError implementation
            ResponseError::error_details(self)
        })
    }
}

pub fn response<Err>(trace_id: &str, err: Err) -> axum::response::Response
where
    Err: ResponseError + ServiceErrorMapping,
{
    // Use user_message if provided, otherwise fallback to thiserror message
    let message = if let Some(user_msg) = err.user_message() {
        user_msg
    } else {
        err.message()
    };

    ApiError {
        trace_id: trace_id.to_string(),
        timestamp: chrono::Utc::now(),
        code: err.error_code(),
        status: err.status_code(),
        message,
        description: err.error_description(),
        details: err.error_details(),
    }
    .into_response()
}
