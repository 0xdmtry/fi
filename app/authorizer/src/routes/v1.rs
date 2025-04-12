use axum::Router;

use super::ping;
use super::join;

pub fn routes() -> Router {
    Router::new()
        .merge(ping::routes()) // All v1 routes go here
        .merge(join::routes()) 
}
