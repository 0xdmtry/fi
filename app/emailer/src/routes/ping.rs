use axum::{Router, routing::get, response::IntoResponse};

pub fn routes() -> Router {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> impl IntoResponse {
    "emailer-v0.2.0"
}
