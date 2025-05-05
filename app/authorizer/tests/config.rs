use authorizer::config::AppConfig;
use serial_test::serial;
use std::env;

fn clear_env() {
    for key in [
        "PASSCODE_TTL_SECONDS",
        "PASSCODE_LEN",
        "PASSCODE_MIN_RANGE",
        "PASSCODE_MAX_RANGE",
        "PASSCODE_MAX_ATTEMPTS",
        "PASSCODE_MAX_RESENDS",
        "DATABASE_URL",
        "DATABASE_TEST_URL",
        "EMAILER_URL",
        "EMAILER_TEST_URL",
        "RUN_MIGRATIONS",
        "DB_CONN_MAX_ATTEMPTS",
        "DB_CONN_RETRY_DELAY_SECONDS",
        "SESSION_TTL_SECONDS",
        "ACCESS_TOKEN_TTL_SECONDS",
        "JWT_SECRET",
    ] {
        unsafe { env::remove_var(key) };
    }
}

#[test]
#[serial]
fn test_defaults_applied() {
    clear_env();
    let config = AppConfig::from_env();

    assert_eq!(config.passcode_ttl_seconds, 300);
    assert_eq!(config.passcode_len, 4);
    assert_eq!(config.passcode_min_range, 1000);
    assert_eq!(config.passcode_max_range, 10000);
    assert_eq!(config.passcode_max_attempts, 3);
    assert_eq!(config.passcode_max_resends, 3);
    assert_eq!(config.database_url, "");
    assert_eq!(config.database_test_url, "");
    assert_eq!(config.emailer_url, "");
    assert_eq!(config.emailer_test_url, "");
    assert!(!config.run_migrations);
    assert_eq!(config.db_conn_max_attempts, 10);
    assert_eq!(config.db_conn_retry_delay_seconds, 2);
    assert_eq!(config.session_ttl_seconds, 604800);
    assert_eq!(config.access_token_ttl_seconds, 900);
    assert_eq!(config.jwt_secret, "");
}

#[test]
#[serial]
fn test_valid_env_values() {
    clear_env();

    unsafe {
        env::set_var("PASSCODE_TTL_SECONDS", "900");
        env::set_var("PASSCODE_LEN", "6");
        env::set_var("PASSCODE_MIN_RANGE", "100000");
        env::set_var("PASSCODE_MAX_RANGE", "1009999");
        env::set_var("PASSCODE_MAX_ATTEMPTS", "5");
        env::set_var("PASSCODE_MAX_RESENDS", "7");
        env::set_var("DATABASE_URL", "postgres://real");
        env::set_var("DATABASE_TEST_URL", "postgres://test");
        env::set_var("EMAILER_URL", "http://emailer.dev");
        env::set_var("EMAILER_TEST_URL", "http://emailer.test");
        env::set_var("RUN_MIGRATIONS", "true");
        env::set_var("DB_CONN_MAX_ATTEMPTS", "20");
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "3");

        env::set_var("SESSION_TTL_SECONDS", "1000");
        env::set_var("ACCESS_TOKEN_TTL_SECONDS", "100");
        env::set_var("JWT_SECRET", "secret");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.passcode_ttl_seconds, 900);
    assert_eq!(config.passcode_len, 6);
    assert_eq!(config.passcode_min_range, 100000);
    assert_eq!(config.passcode_max_range, 1009999);
    assert_eq!(config.passcode_max_attempts, 5);
    assert_eq!(config.passcode_max_resends, 7);
    assert_eq!(config.database_url, "postgres://real");
    assert_eq!(config.database_test_url, "postgres://test");
    assert_eq!(config.emailer_url, "http://emailer.dev");
    assert_eq!(config.emailer_test_url, "http://emailer.test");
    assert!(config.run_migrations);
    assert_eq!(config.db_conn_max_attempts, 20);
    assert_eq!(config.db_conn_retry_delay_seconds, 3);
    assert_eq!(config.session_ttl_seconds, 1000);
    assert_eq!(config.access_token_ttl_seconds, 100);
    assert_eq!(config.jwt_secret, "secret");
}

#[test]
#[serial]
fn test_invalid_len_or_range_group_fallback() {
    clear_env();

    // Invalid len
    unsafe {
        env::set_var("PASSCODE_LEN", "3"); // too small
        env::set_var("PASSCODE_MIN_RANGE", "100");
        env::set_var("PASSCODE_MAX_RANGE", "10000");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.passcode_len, 4);
    assert_eq!(config.passcode_min_range, 1000);
    assert_eq!(config.passcode_max_range, 10000);
}

#[test]
#[serial]
fn test_invalid_narrow_range_fallback() {
    clear_env();

    // Difference is too small: max - min = 1000 (not ≥ 9000)
    unsafe {
        env::set_var("PASSCODE_LEN", "4");
        env::set_var("PASSCODE_MIN_RANGE", "1000");
        env::set_var("PASSCODE_MAX_RANGE", "2000");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.passcode_len, 4);
    assert_eq!(config.passcode_min_range, 1000);
    assert_eq!(config.passcode_max_range, 10000); // fallback kicks in
}

#[test]
#[serial]
fn test_zero_or_invalid_individual_fallbacks() {
    clear_env();

    unsafe {
        env::set_var("PASSCODE_TTL_SECONDS", "0");
        env::set_var("PASSCODE_MAX_ATTEMPTS", "-1");
        env::set_var("PASSCODE_MAX_RESENDS", "0");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.passcode_ttl_seconds, 300);
    assert_eq!(config.passcode_max_attempts, 3);
    assert_eq!(config.passcode_max_resends, 3);
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
fn test_db_conn_config_valid_values() {
    clear_env();

    unsafe {
        env::set_var("DB_CONN_MAX_ATTEMPTS", "15");
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "5");
    }

    let config = AppConfig::from_env();
    assert_eq!(config.db_conn_max_attempts, 15);
    assert_eq!(config.db_conn_retry_delay_seconds, 5);
}

#[test]
#[serial]
fn test_db_conn_config_invalid_values_fallback() {
    clear_env();

    unsafe {
        env::set_var("DB_CONN_MAX_ATTEMPTS", "0");
        env::set_var("DB_CONN_RETRY_DELAY_SECONDS", "-1");
    }

    let config = AppConfig::from_env();
    assert_eq!(config.db_conn_max_attempts, 10);
    assert_eq!(config.db_conn_retry_delay_seconds, 2);
}

#[test]
#[serial]
fn test_session_and_token_defaults() {
    clear_env();

    let config = AppConfig::from_env();

    assert_eq!(config.session_ttl_seconds, 604800); // 7 days
    assert_eq!(config.access_token_ttl_seconds, 900); // 15 minutes
    assert_eq!(config.jwt_secret, "");
}

#[test]
#[serial]
fn test_session_and_token_custom_valid_values() {
    clear_env();

    unsafe {
        env::set_var("SESSION_TTL_SECONDS", "86400"); // 1 day
        env::set_var("ACCESS_TOKEN_TTL_SECONDS", "600"); // 10 min
        env::set_var("JWT_SECRET", "testsecret123");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.session_ttl_seconds, 86400);
    assert_eq!(config.access_token_ttl_seconds, 600);
    assert_eq!(config.jwt_secret, "testsecret123");
}

#[test]
#[serial]
fn test_session_and_token_invalid_values_fallback() {
    clear_env();

    unsafe {
        env::set_var("SESSION_TTL_SECONDS", "0");
        env::set_var("ACCESS_TOKEN_TTL_SECONDS", "-1");
    }

    let config = AppConfig::from_env();

    assert_eq!(config.session_ttl_seconds, 604800); // fallback to 7 days
    assert_eq!(config.access_token_ttl_seconds, 900); // fallback to 15 min
}
