use emailer::config::AppConfig;
use emailer::models::Provider;
use serial_test::serial;
use std::env;

fn clear_env() {
    for key in [
        "EMAIL_PROVIDER",
        "MAX_RETRIES",
        "DB_CONN_MAX_ATTEMPTS",
        "DB_CONN_RETRY_DELAY_SECONDS",
        "DATABASE_URL",
        "DATABASE_TEST_URL",
        "RUN_MIGRATIONS",
        "MAILHOG_SERVER",
        "MAILHOG_PORT",
        "MAILHOG_TEST_SERVER",
        "MAILHOG_TEST_PORT",
        "EMAILER_TEST_URL",
    ] {
        unsafe { env::remove_var(key) };
    }
}

#[test]
#[serial]
fn test_emailer_config_defaults() {
    clear_env();
    let config = AppConfig::from_env();

    assert_eq!(config.email_provider, Provider::Mailhog);
    assert_eq!(config.max_reties, 10);
    assert_eq!(config.db_conn_max_attempts, 10);
    assert_eq!(config.db_conn_retry_delay_seconds, 2);
    assert!(!config.run_migrations);
    assert_eq!(config.database_url, "");
    assert_eq!(config.database_test_url, "");
    assert_eq!(config.mailhog_server, "");
    assert_eq!(config.mailhog_port, 1025);
    assert_eq!(config.mailhog_test_server, "");
    assert_eq!(config.mailhog_test_port, 1125);
    assert_eq!(config.emailer_test_url, "");
}

#[test]
#[serial]
fn test_emailer_config_valid_env_values() {
    clear_env();

    unsafe {
        env::set_var("EMAIL_PROVIDER", "mailhog");
        env::set_var("MAX_RETRIES", "3");
        env::set_var("DB_CONN_MAX_ATTEMPTS", "7");
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "5");
        env::set_var("DATABASE_URL", "postgres://real");
        env::set_var("DATABASE_TEST_URL", "postgres://test");
        env::set_var("RUN_MIGRATIONS", "true");
        env::set_var("MAILHOG_SERVER", "mailhog");
        env::set_var("MAILHOG_PORT", "2025");
        env::set_var("MAILHOG_TEST_SERVER", "localhost");
        env::set_var("MAILHOG_TEST_PORT", "2125");
        env::set_var("EMAILER_TEST_URL", "http://localhost:8101");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.email_provider, Provider::Mailhog);
    assert_eq!(config.max_reties, 3);
    assert_eq!(config.db_conn_max_attempts, 7);
    assert_eq!(config.db_conn_retry_delay_seconds, 5);
    assert_eq!(config.database_url, "postgres://real");
    assert_eq!(config.database_test_url, "postgres://test");
    assert_eq!(config.run_migrations, true);
    assert_eq!(config.mailhog_server, "mailhog");
    assert_eq!(config.mailhog_port, 2025);
    assert_eq!(config.mailhog_test_server, "localhost");
    assert_eq!(config.mailhog_test_port, 2125);
    assert_eq!(config.emailer_test_url, "http://localhost:8101");
}

#[test]
#[serial]
fn test_emailer_invalid_provider_falls_back() {
    clear_env();

    unsafe {
        env::set_var("EMAIL_PROVIDER", "unknown");
    }

    let config = AppConfig::from_env();
    assert_eq!(config.email_provider, Provider::Mailhog);
}

#[test]
#[serial]
fn test_emailer_invalid_retry_counts_fallback() {
    clear_env();

    unsafe {
        env::set_var("MAX_RETRIES", "0");
        env::set_var("DB_CONN_MAX_ATTEMPTS", "-1");
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "0");
        env::set_var("MAILHOG_PORT", "0");
        env::set_var("MAILHOG_TEST_PORT", "0");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.max_reties, 10);
    assert_eq!(config.db_conn_max_attempts, 10);
    assert_eq!(config.db_conn_retry_delay_seconds, 2);
    assert_eq!(config.mailhog_port, 1025);
    assert_eq!(config.mailhog_test_port, 1125);
}

#[test]
#[serial]
fn test_emailer_run_migrations_variants() {
    clear_env();

    unsafe {
        env::set_var("RUN_MIGRATIONS", "1");
        assert!(AppConfig::from_env().run_migrations);

        env::set_var("RUN_MIGRATIONS", "true");
        assert!(AppConfig::from_env().run_migrations);

        env::set_var("RUN_MIGRATIONS", "false");
        assert!(!AppConfig::from_env().run_migrations);

        env::set_var("RUN_MIGRATIONS", "banana");
        assert!(!AppConfig::from_env().run_migrations);
    }
}
