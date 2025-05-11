use super::{ata, ping, sign, signup};
use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(ping::routes())
        .merge(signup::routes())
        .merge(sign::routes())
        .merge(ata::routes())
}
