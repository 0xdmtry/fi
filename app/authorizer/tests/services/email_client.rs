use authorizer::config::AppConfig;
use authorizer::services::email_client;
use serial_test::serial;

fn test_email() -> &'static str {
    "test@example.com"
}

fn test_code() -> &'static str {
    "1234"
}

#[tokio::test]
#[serial]
async fn test_send_passcode_success() {
    let config = AppConfig::from_env_with_custom_file(".test.env");

    let result = email_client::send_passcode_email(&config, test_email(), test_code()).await;

    assert!(
        result.is_ok(),
        "Expected successful email send, got: {result:?}"
    );
}

#[tokio::test]
#[serial]
async fn test_send_passcode_fails_on_404() {
    // Create config with broken route
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = format!("{}/v1/non-existent", config.emailer_url);

    let result = email_client::send_passcode_email(&config, test_email(), test_code()).await;

    assert!(
        result.is_err(),
        "Expected error from invalid emailer URL, got Ok"
    );

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("404") || err.contains("failed"),
        "Unexpected error: {err}"
    );
}

#[tokio::test]
#[serial]
async fn test_send_passcode_fails_on_bad_host() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = "http://127.0.0.1:9999".to_string(); // Unbound port

    let result = email_client::send_passcode_email(&config, test_email(), test_code()).await;

    assert!(
        result.is_err(),
        "Expected connection error due to unreachable host, got Ok"
    );
}

#[tokio::test]
#[serial]
async fn test_send_passcode_fails_on_malformed_url() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.emailer_url = "not-a-url".to_string();

    let result = email_client::send_passcode_email(&config, test_email(), test_code()).await;

    assert!(
        result.is_err(),
        "Expected error due to invalid URL, got Ok"
    );

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("invalid URL") || err.contains("builder error"),
        "Unexpected error: {err}"
    );
}
