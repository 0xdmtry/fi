use crate::handlers::verify_handler::verify_handler;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new().route("/verify", post(verify_handler))
}
