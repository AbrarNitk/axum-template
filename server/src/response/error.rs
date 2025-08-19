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
