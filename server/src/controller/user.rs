use crate::response;
use crate::service;
use crate::service::user::CreateUserReq;
use tracing::{error, instrument};

#[instrument(skip_all, name = "user.get")]
pub async fn get(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    // This is a mock implementation - you would call your actual user service here
    match service::user::get_user(id.clone()).await {
        Ok(user) => response::success(user),
        Err(e) => {
            error!(
                error = %e,
                user_id = %id,
                "Failed to get user"
            );
            response::error::response("user.get", e)
        }
    }
}

#[instrument(skip_all, name = "user.create")]
pub async fn create(
    axum::Json(payload): axum::Json<CreateUserReq>,
) -> impl axum::response::IntoResponse {
    // This is a mock implementation - you would call your actual user service here
    match service::user::create_user(payload.clone()).await {
        Ok(user) => response::success(user),
        Err(e) => {
            error!(
                error = %e,
                user_email = %payload.email,
                "Failed to create user"
            );
            response::error::response("user.create", e)
        }
    }
}
