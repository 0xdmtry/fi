use axum::{Router, routing::post};
use crate::handlers::join_handler::join_handler;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/join", post(join_handler))
}