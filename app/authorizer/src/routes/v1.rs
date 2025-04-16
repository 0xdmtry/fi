use axum::Router;
use super::ping;
use super::join;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(ping::routes()) 
        .merge(join::routes()) 
}
