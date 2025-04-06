use axum::Router;
use crate::routes;
use crate::db::connection;
use sea_orm::DbConn;

pub async fn create_app() -> Router {
    let db: DbConn = connection::establish_connection().await;

    Router::new()
        .nest("/v1", routes::v1::routes()) // version prefix
}
