pub async fn health_router() -> axum::Router {
    axum::Router::new().route(
        "/v1/api/health/",
        axum::routing::get(crate::controller::health::health),
    )
}

pub async fn routes() -> axum::Router {
    axum::Router::new().merge(health_router().await)
}
