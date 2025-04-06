use axum::Router;

use super::ping;

pub fn routes() -> Router {
    Router::new()
        .merge(ping::routes()) // All v1 routes go here
}
