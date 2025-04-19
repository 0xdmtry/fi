use crate::config::AppConfig;
use crate::db::{self, connection};
use crate::routes;
use crate::services::{Emailer, MailhogEmailer};
use crate::state::AppState;
use axum::Router;
use std::sync::Arc;
use crate::models::Provider;

pub async fn create_app() -> Router {
    let config = AppConfig::from_env();
    let db_conn = connection::establish_connection(&config).await;
    let emailer: Arc<dyn Emailer> = match config.email_provider {
        Provider::Mailhog => Arc::new(MailhogEmailer::new()),
        _ => Arc::new(MailhogEmailer::new()),
    };

    let state = AppState { config, db_conn , emailer};

    db::init::run_migrations_if_enabled(&state.db_conn, &state.config).await;

    Router::new()
        .nest("/v1", routes::v1::routes())
        .with_state(state)
}
