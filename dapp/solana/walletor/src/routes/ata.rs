use crate::handlers::ata_handler;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new().route("/create-ata", post(ata_handler::create_ata_handler))
}
