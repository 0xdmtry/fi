use crate::handlers::passcode_handler::{
    send_failed_passcode_handler, send_passcode_handler, send_success_passcode_handler,
};
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/passcode", post(send_passcode_handler))
        .route("/passcode/success", post(send_success_passcode_handler))
        .route("/passcode/failed", post(send_failed_passcode_handler))
}
