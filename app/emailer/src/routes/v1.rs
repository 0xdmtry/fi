use axum::Router;

use super::ping;
use super::passcode;

pub fn routes() -> Router {
    Router::new()
        .merge(ping::routes()) // All v1 routes go here
        .merge(passcode::routes())
}
