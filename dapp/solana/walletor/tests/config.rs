use serial_test::serial;
use std::env;
use walletor::config::AppConfig;

fn clear_env() {
    for key in [
        "DATABASE_URL",
        "DATABASE_TEST_URL",
        "RUN_MIGRATIONS",
        "DB_CONN_MAX_ATTEMPTS",
        "DB_CONN_RETRY_DELAY_SECONDS",
        "WALLETOR_TEST_URL",
    ] {
        unsafe { env::remove_var(key) };
    }
}

#[test]
#[serial]
fn test_defaults_applied() {
    clear_env();
    let config = AppConfig::from_env();

    assert_eq!(config.database_url, "");
    assert_eq!(config.database_test_url, "");
    assert!(!config.run_migrations);
    assert_eq!(config.db_conn_max_attempts, 10);
    assert_eq!(config.db_conn_retry_delay_seconds, 2);
    assert_eq!(config.walletor_test_url, "");
}

#[test]
#[serial]
fn test_valid_env_values() {
    clear_env();

    unsafe {
        env::set_var("DATABASE_URL", "postgres://real");
        env::set_var("DATABASE_TEST_URL", "postgres://test");
        env::set_var("RUN_MIGRATIONS", "true");
        env::set_var("DB_CONN_MAX_ATTEMPTS", "15");
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "5");
        env::set_var("WALLETOR_TEST_URL", "http://localhost:3002");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.database_url, "postgres://real");
    assert_eq!(config.database_test_url, "postgres://test");
    assert!(config.run_migrations);
    assert_eq!(config.db_conn_max_attempts, 15);
    assert_eq!(config.db_conn_retry_delay_seconds, 5);
    assert_eq!(config.walletor_test_url, "http://localhost:3002");
}

#[test]
#[serial]
fn test_run_migrations_variants() {
    clear_env();

    unsafe {
        env::set_var("RUN_MIGRATIONS", "1");
        assert!(AppConfig::from_env().run_migrations);

        env::set_var("RUN_MIGRATIONS", "true");
        assert!(AppConfig::from_env().run_migrations);

        env::set_var("RUN_MIGRATIONS", "false");
        assert!(!AppConfig::from_env().run_migrations);

        env::set_var("RUN_MIGRATIONS", "maybe");
        assert!(!AppConfig::from_env().run_migrations);
    }
}

#[test]
#[serial]
fn test_db_conn_valid_values() {
    clear_env();

    unsafe {
        env::set_var("DB_CONN_MAX_ATTEMPTS", "20");
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "6");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.db_conn_max_attempts, 20);
    assert_eq!(config.db_conn_retry_delay_seconds, 6);
}

#[test]
#[serial]
fn test_db_conn_invalid_values_fallback() {
    clear_env();

    unsafe {
        env::set_var("DB_CONN_MAX_ATTEMPTS", "0"); // invalid
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "0"); // invalid
    }

    let config = AppConfig::from_env();

    assert_eq!(config.db_conn_max_attempts, 10); // default
    assert_eq!(config.db_conn_retry_delay_seconds, 2); // default
}
