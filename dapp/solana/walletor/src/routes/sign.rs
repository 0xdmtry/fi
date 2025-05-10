use crate::handlers::sign_handler;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new().route("/sign", post(sign_handler::sign_transaction_handler))
}
