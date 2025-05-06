use super::join;
use super::ping;
use super::resend;
use super::session;
use super::verify;
use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(ping::routes())
        .merge(join::routes())
        .merge(verify::routes())
        .merge(resend::routes())
        .merge(session::routes())
}
