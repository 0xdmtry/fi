use axum::{Router, Extension};
use crate::routes;
use crate::db::{self, connection};
use sea_orm::DbConn;

pub async fn create_app() -> Router {
    let db_conn: DbConn = connection::establish_connection().await;

    db::init::run_migrations_if_enabled(&db_conn).await;

    Router::new()
        .nest("/v1", routes::v1::routes()) // version prefix
        .layer(Extension(db_conn))
}
