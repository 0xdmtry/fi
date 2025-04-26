use emailer::config::AppConfig;
use emailer::models::passcode::{
    SendFailedPasscodeRequest, SendPasscodeRequest, SendSuccessPasscodeRequest,
};
use reqwest::Client;
use serial_test::serial;
use uuid::Uuid;

async fn setup_config() -> AppConfig {
    AppConfig::from_env_with_custom_file(".test.env")
}

#[tokio::test]
#[serial]
async fn test_valid_passcode_payload_returns_200() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = SendPasscodeRequest {
        email: format!("tester-{}@example.com", Uuid::new_v4()),
        passcode: "1234".into(),
    };

    let res = client
        .post(format!("{}/v1/passcode", config.emailer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
}

#[tokio::test]
#[serial]
async fn test_invalid_email_returns_400() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = SendPasscodeRequest {
        email: "not-an-email".into(),
        passcode: "1234".into(),
    };

    let res = client
        .post(format!("{}/v1/passcode", config.emailer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}

#[tokio::test]
#[serial]
async fn test_passcode_with_letters_returns_400() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = SendPasscodeRequest {
        email: "tester@example.com".into(),
        passcode: "12ab".into(), // ❌ Non-digit
    };

    let res = client
        .post(format!("{}/v1/passcode", config.emailer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}

#[tokio::test]
#[serial]
async fn test_passcode_too_short_returns_400() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = SendPasscodeRequest {
        email: "tester@example.com".into(),
        passcode: "12".into(), // ❌ Too short
    };

    let res = client
        .post(format!("{}/v1/passcode", config.emailer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}

// Success Passcode

#[tokio::test]
#[serial]
async fn test_success_passcode_handler_returns_200() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = SendSuccessPasscodeRequest {
        email: format!("tester-success-{}@example.com", Uuid::new_v4()),
    };

    let res = client
        .post(format!("{}/v1/passcode/success", config.emailer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
}

#[tokio::test]
#[serial]
async fn test_success_passcode_handler_invalid_email_returns_400() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = SendSuccessPasscodeRequest {
        email: "invalid-email".into(),
    };

    let res = client
        .post(format!("{}/v1/passcode/success", config.emailer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}

// Failed Passcode

#[tokio::test]
#[serial]
async fn test_failed_passcode_handler_returns_200() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = SendFailedPasscodeRequest {
        email: format!("tester-failed-{}@example.com", Uuid::new_v4()),
    };

    let res = client
        .post(format!("{}/v1/passcode/failed", config.emailer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
}

#[tokio::test]
#[serial]
async fn test_failed_passcode_handler_invalid_email_returns_400() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = SendFailedPasscodeRequest {
        email: "bad-email".into(),
    };

    let res = client
        .post(format!("{}/v1/passcode/failed", config.emailer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}
