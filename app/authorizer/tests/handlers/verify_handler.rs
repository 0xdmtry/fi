use authorizer::config::AppConfig;
use authorizer::models::join::JoinRequest;
use authorizer::models::verify::VerifyPasscodeRequest;
use reqwest::Client;
use serial_test::serial;
use uuid::Uuid;

use authorizer::models::passcode;
use authorizer::repositories::user_repository;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

async fn setup_config() -> AppConfig {
    AppConfig::from_env_with_custom_file(".test.env")
}

async fn send_join_request(email: &str, config: &AppConfig) {
    let payload = JoinRequest {
        email: email.into(),
    };

    let url = format!("{}/v1/join", config.authorizer_test_url);

    let _ = Client::new().post(url).json(&payload).send().await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_verify_with_valid_code_returns_200() {
    let config = setup_config().await;
    let client = Client::new();

    let email = format!("verify-handler-{}@example.com", Uuid::new_v4());

    send_join_request(&email, &config).await;

    let db = sea_orm::Database::connect(&config.database_test_url)
        .await
        .unwrap();

    let user = user_repository::find_by_email(&db, &email)
        .await
        .unwrap()
        .unwrap();

    let code = passcode::Entity::find()
        .filter(passcode::Column::UserId.eq(user.id))
        .filter(passcode::Column::Used.eq(false))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .code;

    let payload = VerifyPasscodeRequest {
        email: email.clone(),
        passcode: code,
    };

    let url = format!("{}/v1/verify", &config.authorizer_test_url);

    let res = client.post(url).json(&payload).send().await.unwrap();

    assert_eq!(res.status(), 200);
}

#[tokio::test]
#[serial]
async fn test_verify_with_invalid_code_returns_400() {
    let config = setup_config().await;
    let client = Client::new();

    let email = format!("verify-handler-{}@example.com", Uuid::new_v4());

    send_join_request(&email, &config).await;

    let payload = VerifyPasscodeRequest {
        email: email.clone(),
        passcode: "wrong".into(),
    };

    let url = format!("{}/v1/verify", &config.authorizer_test_url);

    let res = client.post(url).json(&payload).send().await.unwrap();

    assert_eq!(res.status(), 400);
}

#[tokio::test]
#[serial]
async fn test_verify_with_invalid_code_returns_401() {
    let config = setup_config().await;
    let client = Client::new();

    let email = format!("verify-handler-{}@example.com", Uuid::new_v4());

    send_join_request(&email, &config).await;

    let payload = VerifyPasscodeRequest {
        email: email.clone(),
        passcode: "12345".into(),
    };

    let url = format!("{}/v1/verify", &config.authorizer_test_url);

    let res = client.post(url).json(&payload).send().await.unwrap();

    assert_eq!(res.status(), 401);
}

#[tokio::test]
#[serial]
async fn test_verify_with_invalid_email_returns_400() {
    let config = setup_config().await;
    let client = Client::new();

    let payload = VerifyPasscodeRequest {
        email: "bad-email".into(),
        passcode: "1234".into(),
    };

    let url = format!("{}/v1/verify", &config.authorizer_test_url);

    let res = client.post(url).json(&payload).send().await.unwrap();

    assert_eq!(res.status(), 400);
}
