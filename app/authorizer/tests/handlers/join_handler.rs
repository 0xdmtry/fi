use reqwest::{Client, StatusCode};
use serde_json::json;
use serial_test::serial;
use uuid::Uuid;
use authorizer::config::AppConfig;

const BASE_URL: &str = "http://localhost:8100/v1/join";

#[tokio::test]
#[serial]
async fn test_join_valid_email_returns_200() {
    let client = Client::new();
    let test_email = format!("test-{}@example.com", Uuid::new_v4());
    let payload = json!({ "email": test_email });

    let res = client.post(BASE_URL).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_join_invalid_email_returns_400() {
    let client = Client::new();
    let payload = json!({ "email": "not-an-email" });

    let res = client.post(BASE_URL).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn test_join_email_too_short_returns_400() {
    let client = Client::new();
    let payload = json!({ "email": "a@b" });

    let res = client.post(BASE_URL).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn test_join_email_too_long_returns_400() {
    let client = Client::new();
    let long_email = format!("{}@example.com", "a".repeat(255));
    let payload = json!({ "email": long_email });

    let res = Client::new()
        .post(BASE_URL)
        .json(&payload)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn test_join_email_with_leading_dot_returns_400() {
    let client = Client::new();
    let payload = json!({ "email": ".user@example.com" });

    let res = client.post(BASE_URL).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn test_join_missing_email_field_returns_422() {
    let client = Client::new();
    let payload = json!({ "foo": "bar" });

    let res = client.post(BASE_URL).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
#[serial]
async fn test_join_empty_body_returns_422() {
    let client = Client::new();

    let res = client
        .post(BASE_URL)
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
    let client = Client::new();
    let test_email = format!("Test-{}@example.com", Uuid::new_v4());
    let payload = json!({ "email": test_email });

    let res = client.post(BASE_URL).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
