use super::ping;
use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().merge(ping::routes()) // All v1 routes go here
}
