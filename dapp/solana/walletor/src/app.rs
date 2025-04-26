use crate::config::AppConfig;
use crate::db::{self, connection};
use crate::routes;
use crate::state::AppState;
use axum::Router;

pub async fn create_app() -> Router {
    let config = AppConfig::from_env();
    let db_conn = connection::establish_connection(&config).await;

    let state = AppState { config, db_conn };

    db::init::run_migrations_if_enabled(&state.db_conn, &state.config).await;

    Router::new()
        .nest("/v1", routes::v1::routes())
        .with_state(state)
}
