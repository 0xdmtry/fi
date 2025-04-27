use crate::handlers::singup_handler;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new().route("/signup", post(singup_handler::signup_wallet_handler))
}
