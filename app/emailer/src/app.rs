use axum::{Router, Extension};
use crate::routes;
use crate::services::{Emailer, MailhogEmailer};
use std::sync::Arc;
use std::env;
use crate::db::{self, connection};
use sea_orm::DbConn;

pub async fn create_app() -> Router {

    let emailer: Arc<dyn Emailer> = match env::var("EMAIL_PROVIDER").as_deref() {
        _ => Arc::new(MailhogEmailer::new()),
    };

    let db_conn: DbConn = connection::establish_connection().await;

    db::init::run_migrations_if_enabled(&db_conn).await;

    Router::new()
        .nest("/v1", routes::v1::routes()) // version prefix
        .layer(Extension(emailer))
}
