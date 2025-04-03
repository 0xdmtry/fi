use axum::Router;
use crate::routes;

pub fn create_app() -> Router {
    Router::new()
        .nest("/v1", routes::v1::routes()) // version prefix
}
