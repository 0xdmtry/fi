use axum::{Router};
use crate::routes;
use crate::db::{self, connection};
use crate::config::AppConfig;
use crate::state::AppState;


pub async fn create_app() -> Router {
    let db_conn = connection::establish_connection().await;
    let config = AppConfig::from_env();

    let state = AppState {
        config,
        db_conn: db_conn.clone(),
    };

    db::init::run_migrations_if_enabled(&db_conn).await;

    Router::new()
        .nest("/v1", routes::v1::routes())
        .with_state(state)
}
