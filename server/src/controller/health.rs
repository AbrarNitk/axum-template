use axum::response::IntoResponse;

pub async fn health() -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::OK,
        "Server is working fine, you can send the traffic",
    )
        .into_response()
}
