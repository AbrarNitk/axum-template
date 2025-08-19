pub mod error;

#[derive(serde::Serialize)]
pub struct ApiErrorResponse {
    success: bool,
    error: error::ApiError,
}
