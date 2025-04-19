use axum::{Router, routing::post};
use crate::handlers::passcode_handler::send_passcode_handler;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/passcode", post(send_passcode_handler))
}