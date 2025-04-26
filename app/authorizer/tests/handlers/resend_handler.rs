use authorizer::config::AppConfig;
use authorizer::models::resend::ResendRequest;
use reqwest::Client;
use serial_test::serial;
use uuid::Uuid;

async fn setup_config() -> AppConfig {
    AppConfig::from_env_with_custom_file(".test.env")
}

#[tokio::test]
#[serial]
async fn test_resend_valid_email_returns_200() {
    let config = setup_config().await;
    let client = Client::new();

    // Send join request first to create user & passcode
    let join_payload = serde_json::json!({
        "email": format!("resend-valid-{}@example.com", Uuid::new_v4())
    });

    client
        .post(format!("{}/v1/join", config.authorizer_test_url))
        .json(&join_payload)
        .send()
        .await
        .unwrap();

    let resend_payload = ResendRequest {
        email: join_payload["email"].as_str().unwrap().to_string(),
    };

    let res = client
        .post(format!("{}/v1/resend", config.authorizer_test_url))
        .json(&resend_payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
}

#[tokio::test]
#[serial]
async fn test_resend_invalid_email_format_returns_400() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = ResendRequest {
        email: "bad-email".into(),
    };

    let res = client
        .post(format!("{}/v1/resend", config.authorizer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
}

#[tokio::test]
#[serial]
async fn test_resend_nonexistent_email_returns_500() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = ResendRequest {
        email: format!("no-user-{}@example.com", Uuid::new_v4()),
    };

    let res = client
        .post(format!("{}/v1/resend", config.authorizer_test_url))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 500);
}
