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
    message: String,
    description: Option<String>,
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

    fn desc(&self) -> Option<String> {
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
        message: err.message(),
        description: err.desc(),
    }
    .into_response()
}
