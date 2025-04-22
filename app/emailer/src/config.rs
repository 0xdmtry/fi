use crate::models::Provider;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub email_provider: Provider,

    pub max_reties: i32,

    pub db_conn_max_attempts: u32,
    pub db_conn_retry_delay_seconds: u64,

    pub run_migrations: bool,

    pub database_url: String,
    pub database_test_url: String,

    pub mailhog_server: String,
    pub mailhog_port: u16,

    pub mailhog_test_server: String,
    pub mailhog_test_port: u16,

    pub emailer_test_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let default_max_retries: i32 = 10;

        let default_db_conn_max_attempts: u32 = 10;

        let default_mailhog_port: u16 = 1025;
        let default_mailhog_test_port: u16 = 1125;

        let default_db_conn_retry_delay_seconds: u64 = 2;

        let email_provider = env::var("EMAIL_PROVIDER")
            .ok()
            .and_then(|v| v.parse::<Provider>().ok())
            .unwrap_or_default();

        let reties = env::var("MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse::<i32>().ok());

        let max_reties = match (reties) {
            (Some(retries)) if retries > 0 => retries,
            _ => default_max_retries,
        };

        let db_max_attempts = env::var("DB_CONN_MAX_ATTEMPTS")
            .ok()
            .and_then(|v| v.parse::<u32>().ok());

        let db_conn_max_attempts: u32 = match (db_max_attempts) {
            (Some(attempts)) if attempts > 0 => attempts,
            _ => default_db_conn_max_attempts,
        };

        let db_retry_delay_seconds = env::var("DB_CONN_RETRY_DELAY_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());

        let db_conn_retry_delay_seconds = match (db_retry_delay_seconds) {
            (Some(seconds)) if seconds > 0 => seconds,
            _ => default_db_conn_retry_delay_seconds,
        };

        let run_migrations = env::var("RUN_MIGRATIONS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let database_url = env::var("DATABASE_URL").unwrap_or_default();
        let database_test_url = env::var("DATABASE_TEST_URL").unwrap_or_default();

        let mailhog_server = env::var("MAILHOG_SERVER").unwrap_or_default();
        let mailhog_port_value = env::var("MAILHOG_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok());
        let mailhog_port = match (mailhog_port_value) {
            (Some(port)) if port > 0 => port,
            _ => default_mailhog_port,
        };

        let mailhog_test_server = env::var("MAILHOG_TEST_SERVER").unwrap_or_default();
        let mailhog_test_port_value = env::var("MAILHOG_TEST_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok());
        let mailhog_test_port = match (mailhog_test_port_value) {
            (Some(port)) if port > 0 => port,
            _ => default_mailhog_test_port,
        };

        let emailer_test_url = env::var("EMAILER_TEST_URL").unwrap_or_default();

        Self {
            email_provider,

            max_reties,

            db_conn_max_attempts,
            db_conn_retry_delay_seconds,

            run_migrations,

            database_url,
            database_test_url,

            mailhog_server,
            mailhog_port,

            mailhog_test_server,
            mailhog_test_port,

            emailer_test_url,
        }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename(file_name).ok();
        Self::from_env()
    }
}
