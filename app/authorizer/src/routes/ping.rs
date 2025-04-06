use axum::{Router, routing::get, response::IntoResponse};

pub fn routes() -> Router {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> impl IntoResponse {
    "authorizer-v0.1.0"
}
