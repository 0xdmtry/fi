use axum::{Router, routing::post};
use crate::handlers::passcode::send_passcode_handler;

pub fn routes() -> Router {
    Router::new().route("/passcode", post(send_passcode_handler))
}