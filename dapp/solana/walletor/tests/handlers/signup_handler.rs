use reqwest::{Client, StatusCode};
use serde_json::json;
use serial_test::serial;
use uuid::Uuid;
use walletor::config::AppConfig;

#[tokio::test]
#[serial]
async fn test_signup_valid_user_returns_200() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/signup", config.walletor_test_url);

    let client = Client::new();
    let test_user_id = Uuid::new_v4();
    let payload = json!({
        "user_id": test_user_id,
        "passcode": "1234"
    });

    let res = client.post(&url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_signup_missing_passcode_returns_422() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/signup", config.walletor_test_url);

    let client = Client::new();
    let test_user_id = Uuid::new_v4();
    let payload = json!({
        "user_id": test_user_id
        // Missing "passcode"
    });

    let res = client.post(&url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
#[serial]
async fn test_signup_missing_user_id_returns_422() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/signup", config.walletor_test_url);

    let client = Client::new();
    let payload = json!({
        "passcode": "1234"
        // Missing "user_id"
    });

    let res = client.post(&url).json(&payload).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
#[serial]
async fn test_signup_empty_body_returns_422() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/signup", config.walletor_test_url);

    let client = Client::new();
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
#[serial]
async fn test_signup_invalid_uuid_returns_422() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let url = format!("{}/v1/signup", config.walletor_test_url);

    let client = Client::new();
    let payload = r#"
    {
        "user_id": "not-a-uuid",
        "passcode": "1234"
    }
    "#;

    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
