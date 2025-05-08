use serial_test::serial;
use std::env;
use transactor::config::AppConfig;

fn clear_env() {
    for key in [
        "MAX_RETRIES",
        "DB_CONN_MAX_ATTEMPTS",
        "DB_CONN_RETRY_DELAY_SECONDS",
        "DATABASE_URL",
        "DATABASE_TEST_URL",
        "RUN_MIGRATIONS",
    ] {
        unsafe { env::remove_var(key) };
    }
}

#[test]
#[serial]
fn test_emailer_config_defaults() {
    clear_env();
    let config = AppConfig::from_env();

    assert_eq!(config.max_reties, 10);
    assert_eq!(config.db_conn_max_attempts, 10);
    assert_eq!(config.db_conn_retry_delay_seconds, 2);
    assert!(!config.run_migrations);
    assert_eq!(config.database_url, "");
    assert_eq!(config.database_test_url, "");
}

#[test]
#[serial]
fn test_emailer_config_valid_env_values() {
    clear_env();

    unsafe {
        env::set_var("MAX_RETRIES", "3");
        env::set_var("DB_CONN_MAX_ATTEMPTS", "7");
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "5");
        env::set_var("DATABASE_URL", "postgres://real");
        env::set_var("DATABASE_TEST_URL", "postgres://test");
        env::set_var("RUN_MIGRATIONS", "true");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.max_reties, 3);
    assert_eq!(config.db_conn_max_attempts, 7);
    assert_eq!(config.db_conn_retry_delay_seconds, 5);
    assert_eq!(config.database_url, "postgres://real");
    assert_eq!(config.database_test_url, "postgres://test");
    assert_eq!(config.run_migrations, true);
}

#[test]
#[serial]
fn test_emailer_invalid_retry_counts_fallback() {
    clear_env();

    unsafe {
        env::set_var("MAX_RETRIES", "0");
        env::set_var("DB_CONN_MAX_ATTEMPTS", "-1");
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "0");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.max_reties, 10);
    assert_eq!(config.db_conn_max_attempts, 10);
    assert_eq!(config.db_conn_retry_delay_seconds, 2);
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
