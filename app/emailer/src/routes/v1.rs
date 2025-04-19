use axum::Router;
use crate::state::AppState;
use super::ping;
use super::passcode;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(ping::routes()) // All v1 routes go here
        .merge(passcode::routes())
}
