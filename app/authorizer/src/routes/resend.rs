use axum::{Router, routing::post};
use crate::handlers::resend_handler::resend_handler;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/resend", post(resend_handler))
}