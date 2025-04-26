use axum::Router;
use super::ping;
use super::join;
use super::verify;
use super::resend;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(ping::routes()) 
        .merge(join::routes()) 
        .merge(verify::routes())
        .merge(resend::routes())
}
