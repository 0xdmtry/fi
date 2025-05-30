use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub database_test_url: String,

    pub run_migrations: bool,

    pub emailer_url: String,
    pub emailer_test_url: String,

    pub walletor_url: String,
    pub walletor_test_url: String,

    pub authorizer_test_url: String,

    pub passcode_ttl_seconds: i64,

    pub passcode_len: u32,

    pub passcode_min_range: u64,
    pub passcode_max_range: u64,

    pub passcode_max_attempts: u32,
    pub passcode_max_resends: u32,

    pub db_conn_max_attempts: u32,
    pub db_conn_retry_delay_seconds: u64,

    pub session_ttl_seconds: i64,
    pub access_token_ttl_seconds: i64,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let default_len = 4;

        let default_min_range = 10u64.pow(default_len - 1);
        let default_max_range = 10u64.pow(default_len);

        let min_diff = 9000;

        let default_ttl_seconds = 300;
        let default_max_attempts = 3;
        let default_max_resends = 3;

        let default_db_conn_max_attempts: u32 = 10;
        let default_db_conn_retry_delay_seconds: u64 = 2;

        let default_session_ttl_seconds: i64 = 604800; // 7 days
        let default_access_token_ttl_seconds: i64 = 900; // 15 minutes

        let len = env::var("PASSCODE_LEN")
            .ok()
            .and_then(|v| v.parse::<u32>().ok());

        let min_range = env::var("PASSCODE_MIN_RANGE")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());

        let max_range = env::var("PASSCODE_MAX_RANGE")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());

        let ttl_seconds = env::var("PASSCODE_TTL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok());

        let max_attempts = env::var("PASSCODE_MAX_ATTEMPTS")
            .ok()
            .and_then(|v| v.parse::<u32>().ok());

        let max_resends = env::var("PASSCODE_MAX_RESENDS")
            .ok()
            .and_then(|v| v.parse::<u32>().ok());

        let db_max_attempts = env::var("DB_CONN_MAX_ATTEMPTS")
            .ok()
            .and_then(|v| v.parse::<u32>().ok());

        let db_retry_delay_seconds = env::var("DB_CONN_RETRY_DELAY_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());

        let session_seconds = env::var("SESSION_TTL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok());

        let token_seconds = env::var("ACCESS_TOKEN_TTL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok());

        let (passcode_len, passcode_min_range, passcode_max_range) = match (
            len, min_range, max_range,
        ) {
            (Some(len), Some(min), Some(max))
                if (4..=8).contains(&len)
                    && min >= 10u64.pow(len - 1)
                    && max >= min
                    && max >= 10u64.pow(len)
                    && (max - min) >= min_diff =>
            {
                (len, min, max)
            }
            _ => {
                eprintln!(
                    "⚠️ Invalid passcode config detected: PASSCODE_LEN={:?}, MIN={:?}, MAX={:?}. Falling back to defaults.",
                    len, min_range, max_range
                );
                (default_len, default_min_range, default_max_range)
            }
        };

        let passcode_ttl_seconds = match ttl_seconds {
            Some(seconds) if seconds > 0 => seconds,
            _ => default_ttl_seconds,
        };

        let passcode_max_attempts = match max_attempts {
            Some(attempts) if attempts > 0 => attempts,
            _ => default_max_attempts,
        };

        let passcode_max_resends = match max_resends {
            Some(resends) if resends > 0 => resends,
            _ => default_max_resends,
        };

        let db_conn_max_attempts: u32 = match db_max_attempts {
            Some(attempts) if attempts > 0 => attempts,
            _ => default_db_conn_max_attempts,
        };

        let db_conn_retry_delay_seconds = match db_retry_delay_seconds {
            Some(seconds) if seconds > 0 => seconds,
            _ => default_db_conn_retry_delay_seconds,
        };

        let database_url = env::var("DATABASE_URL").unwrap_or_default();
        let database_test_url = env::var("DATABASE_TEST_URL").unwrap_or_default();
        let run_migrations = env::var("RUN_MIGRATIONS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        let emailer_url = env::var("EMAILER_URL").unwrap_or_default();
        let emailer_test_url = env::var("EMAILER_TEST_URL").unwrap_or_default();

        let walletor_url = env::var("WALLETOR_URL").unwrap_or_default();
        let walletor_test_url = env::var("WALLETOR_TEST_URL").unwrap_or_default();

        let authorizer_test_url = env::var("AUTHORIZER_TEST_URL").unwrap_or_default();

        let session_ttl_seconds: i64 = match session_seconds {
            Some(seconds) if seconds > 0 => seconds,
            _ => default_session_ttl_seconds,
        };
        let access_token_ttl_seconds: i64 = match token_seconds {
            Some(seconds) if seconds > 0 => seconds,
            _ => default_access_token_ttl_seconds,
        };

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_default();

        Self {
            passcode_ttl_seconds,
            passcode_len,
            passcode_min_range,
            passcode_max_range,

            passcode_max_attempts,
            passcode_max_resends,

            db_conn_max_attempts,
            db_conn_retry_delay_seconds,

            database_url,
            database_test_url,

            run_migrations,

            emailer_url,
            emailer_test_url,

            walletor_url,
            walletor_test_url,

            authorizer_test_url,

            session_ttl_seconds,
            access_token_ttl_seconds,
            jwt_secret,
        }
    }

    pub fn from_env_with_custom_file(file_name: &str) -> Self {
        dotenvy::from_filename(file_name).ok();
        Self::from_env()
    }
}
