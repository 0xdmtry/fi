use crate::config::AppConfig;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_conn: DatabaseConnection,
}
