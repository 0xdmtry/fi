use crate::handlers::fund_handler;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new().route("/fund-wsol", post(fund_handler::fund_wsol_handler))
}
