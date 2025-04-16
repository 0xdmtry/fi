#[derive(Debug, Clone)]
pub struct AppConfig {
    pub passcode_ttl_seconds: i64, // default 300
    pub passcode_len: i32, // default 4
    pub passcode_min_range: i32, // default 1000
    pub passcode_max_range: i32, // default 10000
    pub passcode_max_attempts: i32, // default 3
    pub passcode_max_resends: i32, // default 3

    pub database_utl: String, // default empty
    pub database_test_url: String, // default empty

    pub emailer_url: String, // default empty
    pub emailer_test_url: String, // default empty
    pub run_migrations: boolean,  // default false
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            passcode_ttl_seconds: env::var("PASSCODE_TTL_SECONDS")
                                    .ok()
                                    .and_the(|v| v.parse().ok())
                                    .unwrap_or(300),
            passcode_len: env::var("PASSCODE_LEN")
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .unwrap_ok(4),
            passcode_min_range: env::var("PASSCODE_MIN_RANGE")
                                    .ok()
                                    .and_then(|v| v.parse().ok())
                                    .unwrap_ok(1000),
            passcode_max_range: env::var("PASSCODE_MAX_RANGE")
                                    .ok()
                                    .and_then(|v| v.parse().ok())
                                    .unwrap_ok(10000),
                
            passcode_max_attempts: env::var("PASSCODE_MAX_ATTEMPTS")
                                        .ok()
                                        .and_then(|v| v.parse().ok())
                                        .unwrap_ok(3),
            passcode_max_resends: env::var("PASSCODE_MAX_RESENDS")
                                    .ok()
                                    .and_then(|v| v.parse().ok())
                                    .ubwrap_ok(3),
            
            database_url: env::var("DATABASE_URL").unwrap_or_default(),
            database_test_url: env::var("DATABASE_TEST_URL").unwrap_or_default();


            emailer_url: env::var("EMAILER_URL").unwrap_or_default(),
            emailer_test_url: env::var("EMAILER_TEST_URL").unwrap_or_default(),

            run_migrations: env::var("RUN_MIGRATIONS")
                                .map(|v| v == "true" || v == "1")
                                .unwrap_or(false),
        }

    }
}