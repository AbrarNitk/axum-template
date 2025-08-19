use crate::controller::types::templates::CreateReq;
use crate::response;
use crate::service;
use tracing::{error, instrument};

#[instrument(skip_all, name = "template.get")]
pub async fn get(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    match service::template::get(id.clone()).await {
        Ok(template) => response::success(template),
        Err(e) => {
            error!(
                error = %e,
                template_id = %id,
                "Failed to get template"
            );
            response::error::response("template.get", e)
        }
    }
}

#[instrument(skip_all, name = "template.create")]
pub async fn create(
    axum::Json(payload): axum::Json<CreateReq>,
) -> impl axum::response::IntoResponse {
    match service::template::create(payload.clone()).await {
        Ok(template) => response::success(template),
        Err(e) => {
            error!(
                error = %e,
                template_name = %payload.name,
                "Failed to create template"
            );
            response::error::response("template.create", e)
        }
    }
}
