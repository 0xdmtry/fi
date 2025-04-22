use authorizer::config::AppConfig;
use reqwest::{Client, StatusCode};
use serde_json::json;
use serial_test::serial;
use uuid::Uuid;

#[tokio::test]
#[serial]
async fn test_join_valid_email_returns_200() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/join", config.authorizer_test_url);

    let client = Client::new();
    let test_email = format!("test-{}@example.com", Uuid::new_v4());
    let payload = json!({ "email": test_email });

    let res = client.post(url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_join_invalid_email_returns_400() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/join", config.authorizer_test_url);

    let client = Client::new();
    let payload = json!({ "email": "not-an-email" });

    let res = client.post(url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn test_join_email_too_short_returns_400() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/join", config.authorizer_test_url);

    let client = Client::new();
    let payload = json!({ "email": "a@b" });

    let res = client.post(url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn test_join_email_too_long_returns_400() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/join", config.authorizer_test_url);

    let client = Client::new();
    let long_email = format!("{}@example.com", "a".repeat(255));
    let payload = json!({ "email": long_email });

    let res = Client::new().post(url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn test_join_email_with_leading_dot_returns_400() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/join", config.authorizer_test_url);

    let client = Client::new();
    let payload = json!({ "email": ".user@example.com" });

    let res = client.post(url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn test_join_missing_email_field_returns_422() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/join", config.authorizer_test_url);

    let client = Client::new();
    let payload = json!({ "foo": "bar" });

    let res = client.post(url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
#[serial]
async fn test_join_empty_body_returns_422() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/join", config.authorizer_test_url);

    let client = Client::new();

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
#[serial]
async fn test_join_uppercase_email_normalized_returns_200() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/join", config.authorizer_test_url);

    let client = Client::new();
    let test_email = format!("Test-{}@example.com", Uuid::new_v4());
    let payload = json!({ "email": test_email });

    let res = client.post(url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_join_same_email_twice_returns_200() {
    let mut config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/join", config.authorizer_test_url);

    let client = Client::new();
    let test_email = format!("repeat-{}@example.com", Uuid::new_v4());
    let payload = json!({ "email": test_email });

    // First request
    let res1 = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .expect("first join failed");

    assert_eq!(res1.status(), StatusCode::OK);

    // Second request with same email
    let res2 = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .expect("second join failed");

    assert_eq!(res2.status(), StatusCode::OK);
}
