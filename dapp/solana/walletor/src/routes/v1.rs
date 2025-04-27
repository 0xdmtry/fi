use super::{ping, signup};
use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().merge(ping::routes()).merge(signup::routes())
}
