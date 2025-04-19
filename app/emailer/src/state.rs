use crate::config::AppConfig;
use crate::services::Emailer;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_conn: DatabaseConnection,
    pub emailer: Arc<dyn Emailer>,
}
