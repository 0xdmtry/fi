use axum::{Router, routing::get, response::IntoResponse};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> impl IntoResponse {
    "authorizer-v0.3.6"
}
