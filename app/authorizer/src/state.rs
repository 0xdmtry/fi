use sea_orm::DatabaseConnection;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_conn: DatabaseConnection,
}