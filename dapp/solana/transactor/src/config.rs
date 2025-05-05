use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub max_reties: i32,

    pub db_conn_max_attempts: u32,
    pub db_conn_retry_delay_seconds: u64,

    pub run_migrations: bool,

    pub database_url: String,
    pub database_test_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let default_max_retries: i32 = 10;

        let default_db_conn_max_attempts: u32 = 10;

        let default_db_conn_retry_delay_seconds: u64 = 2;

        let reties = env::var("MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse::<i32>().ok());

        let max_reties = match reties {
            Some(retries) if retries > 0 => retries,
            _ => default_max_retries,
        };

        let db_max_attempts = env::var("DB_CONN_MAX_ATTEMPTS")
            .ok()
            .and_then(|v| v.parse::<u32>().ok());

        let db_conn_max_attempts: u32 = match db_max_attempts {
            Some(attempts) if attempts > 0 => attempts,
            _ => default_db_conn_max_attempts,
        };

        let db_retry_delay_seconds = env::var("DB_CONN_RETRY_DELAY_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());

        let db_conn_retry_delay_seconds = match db_retry_delay_seconds {
            Some(seconds) if seconds > 0 => seconds,
            _ => default_db_conn_retry_delay_seconds,
        };

        let run_migrations = env::var("RUN_MIGRATIONS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let database_url = env::var("DATABASE_URL").unwrap_or_default();
        let database_test_url = env::var("DATABASE_TEST_URL").unwrap_or_default();

        Self {
            max_reties,

            db_conn_max_attempts,
            db_conn_retry_delay_seconds,

            run_migrations,

            database_url,
            database_test_url,
        }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename(file_name).ok();
        Self::from_env()
    }
}
