use axum::{Router, routing::post};

use crate::handlers::session_handler::{logout_handler, refresh_handler};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/refresh", post(refresh_handler))
        .route("/auth/logout", post(logout_handler))
}
