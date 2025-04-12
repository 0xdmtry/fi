use axum::{Router, routing::post, response::IntoResponse};
use crate::handlers::join_handler::join_handler;

pub fn routes() -> Router {
    Router::new().route("/join", post(join_handler))
}