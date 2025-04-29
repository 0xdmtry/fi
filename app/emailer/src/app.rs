use crate::config::AppConfig;
use crate::db::{self, connection};
use crate::models::Provider;
use crate::routes;
use crate::routes::metrics_routes;
use crate::services::{Emailer, MailhogEmailer};
use crate::state::AppState;
use axum::Router;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_exporter_prometheus::PrometheusHandle;
use std::sync::Arc;

pub async fn create_app() -> Router {
    let config = AppConfig::from_env();
    let db_conn = connection::establish_connection(&config).await;
    let emailer: Arc<dyn Emailer> = match config.email_provider {
        Provider::Mailhog => Arc::new(MailhogEmailer::new()),
    };

    let prometheus_handle: PrometheusHandle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder");

    let state = AppState {
        config,
        db_conn,
        emailer,
    };

    db::init::run_migrations_if_enabled(&state.db_conn, &state.config).await;

    Router::new()
        .nest("/v1", routes::v1::routes())
        .merge(metrics_routes(prometheus_handle))
        .with_state(state)
}
