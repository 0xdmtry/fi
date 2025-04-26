use authorizer::config::AppConfig;
use authorizer::services::email_client;
use serial_test::serial;

fn test_email() -> &'static str {
    "test@example.com"
}

fn test_code() -> &'static str {
    "1234"
}

async fn setup_emailer_url() -> AppConfig {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = config.emailer_test_url.clone();
    config
}

#[tokio::test]
#[serial]
async fn test_send_passcode_success() {
    let config = setup_emailer_url().await;

    let result = email_client::send_passcode_email(&config, test_email(), test_code()).await;

    assert!(
        result.is_ok(),
        "Expected successful email send, got: {result:?}"
    );
}

#[tokio::test]
#[serial]
async fn test_send_success_passcode_success() {
    let config = setup_emailer_url().await;

    let result = email_client::send_success_passcode_email(&config, test_email()).await;

    assert!(
        result.is_ok(),
        "Expected successful success-passcode email, got: {result:?}"
    );
}

#[tokio::test]
#[serial]
async fn test_send_failed_passcode_success() {
    let config = setup_emailer_url().await;

    let result = email_client::send_failed_passcode_email(&config, test_email()).await;

    assert!(
        result.is_ok(),
        "Expected successful failed-passcode email, got: {result:?}"
    );
}

// Failing cases

#[tokio::test]
#[serial]
async fn test_send_passcode_fails_on_404() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = format!("{}/v1/non-existent", config.emailer_test_url);

    let result = email_client::send_passcode_email(&config, test_email(), test_code()).await;

    assert!(
        result.is_err(),
        "Expected 404 error for wrong endpoint, got Ok"
    );
}

#[tokio::test]
#[serial]
async fn test_send_success_passcode_fails_on_404() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = format!("{}/v1/non-existent", config.emailer_test_url);

    let result = email_client::send_success_passcode_email(&config, test_email()).await;

    assert!(
        result.is_err(),
        "Expected 404 error for wrong success endpoint, got Ok"
    );
}

#[tokio::test]
#[serial]
async fn test_send_failed_passcode_fails_on_404() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = format!("{}/v1/non-existent", config.emailer_test_url);

    let result = email_client::send_failed_passcode_email(&config, test_email()).await;

    assert!(
        result.is_err(),
        "Expected 404 error for wrong failed endpoint, got Ok"
    );
}

#[tokio::test]
#[serial]
async fn test_send_passcode_fails_on_bad_host() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = "http://127.0.0.1:9999".to_string(); // Unreachable

    let result = email_client::send_passcode_email(&config, test_email(), test_code()).await;

    assert!(result.is_err(), "Expected connection error, got Ok");
}

#[tokio::test]
#[serial]
async fn test_send_success_passcode_fails_on_bad_host() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = "http://127.0.0.1:9999".to_string(); // Unreachable

    let result = email_client::send_success_passcode_email(&config, test_email()).await;

    assert!(result.is_err(), "Expected connection error, got Ok");
}

#[tokio::test]
#[serial]
async fn test_send_failed_passcode_fails_on_bad_host() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = "http://127.0.0.1:9999".to_string(); // Unreachable

    let result = email_client::send_failed_passcode_email(&config, test_email()).await;

    assert!(result.is_err(), "Expected connection error, got Ok");
}

#[tokio::test]
#[serial]
async fn test_send_passcode_fails_on_malformed_url() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = "not-a-valid-url".to_string();

    let result = email_client::send_passcode_email(&config, test_email(), test_code()).await;

    assert!(result.is_err(), "Expected error due to invalid URL, got Ok");
}
