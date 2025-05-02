use authorizer::config::AppConfig;
use authorizer::services::walletor_client;
use serial_test::serial;
use uuid::Uuid;

fn test_code() -> &'static str {
    "1234"
}

async fn setup_walletor_test_url() -> AppConfig {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.walletor_url = config.walletor_test_url.clone();
    config
}

#[tokio::test]
#[serial]
async fn test_send_walletor_signup_success() {
    let config = setup_walletor_test_url().await;

    let result =
        walletor_client::send_walletor_signup_request(&config, Uuid::new_v4(), test_code()).await;

    assert!(
        result.is_ok(),
        "Expected walletor signup success, got: {result:?}"
    );
}

#[tokio::test]
#[serial]
async fn test_send_walletor_signup_fails_on_404() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.walletor_url = format!("{}/v1/does-not-exist", config.walletor_test_url);

    let result =
        walletor_client::send_walletor_signup_request(&config, Uuid::new_v4(), test_code()).await;

    assert!(
        result.is_err(),
        "Expected 404 error for wrong Walletor endpoint, got Ok"
    );
}

#[tokio::test]
#[serial]
async fn test_send_walletor_signup_fails_on_bad_host() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.walletor_url = "http://127.0.0.1:9999".to_string(); // Unreachable

    let result =
        walletor_client::send_walletor_signup_request(&config, Uuid::new_v4(), test_code()).await;

    assert!(
        result.is_err(),
        "Expected connection error to bad Walletor host, got Ok"
    );
}

#[tokio::test]
#[serial]
async fn test_send_walletor_signup_fails_on_malformed_url() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    config.walletor_url = "not-a-valid-url".to_string();

    let result =
        walletor_client::send_walletor_signup_request(&config, Uuid::new_v4(), test_code()).await;

    assert!(
        result.is_err(),
        "Expected malformed Walletor URL error, got Ok"
    );
}
