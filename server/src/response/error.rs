use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(serde::Serialize)]
pub enum ErrorCode {
    NotFound,
    InternalServerError,
    BadRequest,
    UnAuthorized,
}

// Implement conversion from ErrorCode to StatusCode
impl From<ErrorCode> for StatusCode {
    fn from(error_code: ErrorCode) -> Self {
        match error_code {
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::BadRequest => StatusCode::BAD_REQUEST,
            ErrorCode::UnAuthorized => StatusCode::UNAUTHORIZED,
            ErrorCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
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
    fn error_code(&self) -> ErrorCode;

    // Automatically derive status code from error code
    // Can be overridden for special cases
    fn status_code(&self) -> StatusCode {
        self.error_code().into()
    }

    // User-friendly message (can override thiserror message)
    fn user_message(&self) -> String {
        self.to_string() // Default: use thiserror message
    }

    // Technical context (optional)
    fn technical_description(&self) -> Option<String> {
        None
    }

    // Full technical details (optional, defaults to backtrace)
    fn technical_details(&self) -> Option<String> {
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

pub fn response<Err>(trace_id: &str, err: Err) -> axum::response::Response
where
    Err: ResponseError,
{
    ApiError {
        trace_id: trace_id.to_string(),
        timestamp: chrono::Utc::now(),
        code: err.error_code(),
        status: err.status_code(),
        message: err.user_message(),
        description: err.technical_description(),
        details: err.technical_details(),
    }
    .into_response()
}
