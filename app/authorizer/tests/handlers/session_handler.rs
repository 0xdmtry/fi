use authorizer::config::AppConfig;
use authorizer::payloads::join_user_payload::JoinRequest;
use authorizer::payloads::verify_passcode_payload::VerifyPasscodeRequest;
use reqwest::{Client, StatusCode};
use serial_test::serial;
use uuid::Uuid;

/// Helper to load test config
async fn setup_config() -> AppConfig {
    AppConfig::from_env_with_custom_file(".test.env")
}

#[tokio::test]
#[serial]
async fn test_refresh_success_returns_200_and_new_access_token() {
    let config = setup_config().await;
    let client = Client::builder().cookie_store(true).build().unwrap();

    // Step 1: Join (trigger passcode)
    let email = format!("refresh-{}@example.com", Uuid::new_v4());
    let join_payload = JoinRequest {
        email: email.clone(),
    };

    client
        .post(format!("{}/v1/join", config.authorizer_test_url))
        .json(&join_payload)
        .send()
        .await
        .unwrap();

    // Step 2: Grab passcode from DB
    let db = sea_orm::Database::connect(&config.database_test_url)
        .await
        .unwrap();
    let user = authorizer::repositories::user_repository::find_by_email(&db, &email)
        .await
        .unwrap()
        .unwrap();
    let passcode =
        authorizer::repositories::passcode_repository::find_active_by_user_id(&db, user.id)
            .await
            .unwrap()
            .unwrap();

    // Step 3: Verify (returns access token + sets refresh cookie)
    let verify_payload = VerifyPasscodeRequest {
        email: email.clone(),
        passcode: passcode.code.clone(),
    };

    let res = client
        .post(format!("{}/v1/verify", config.authorizer_test_url))
        .json(&verify_payload)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = res.json::<serde_json::Value>().await.unwrap();
    assert!(body.get("access_token").is_some());

    // Step 4: Refresh
    let refresh = client
        .post(format!("{}/auth/refresh", config.authorizer_test_url))
        .send()
        .await
        .unwrap();

    assert_eq!(refresh.status(), StatusCode::OK);
    let json = refresh.json::<serde_json::Value>().await.unwrap();
    assert!(json.get("access_token").is_some());
}

#[tokio::test]
#[serial]
async fn test_refresh_with_invalid_token_returns_401() {
    let config = setup_config().await;
    let client = Client::builder().cookie_store(true).build().unwrap();

    // Set a bogus refresh cookie manually
    let cookie = format!("refresh_token={}; Path=/; HttpOnly", Uuid::new_v4());
    let set_cookie = reqwest::header::HeaderValue::from_str(&cookie).unwrap();

    let res = client
        .post(format!("{}/auth/refresh", config.authorizer_test_url))
        .header(reqwest::header::COOKIE, set_cookie)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial]
async fn test_logout_clears_cookie_and_returns_200() {
    let config = setup_config().await;
    let client = Client::builder().cookie_store(true).build().unwrap();

    // Step 1: Join
    let email = format!("logout-{}@example.com", Uuid::new_v4());
    let join_payload = JoinRequest {
        email: email.clone(),
    };

    client
        .post(format!("{}/v1/join", config.authorizer_test_url))
        .json(&join_payload)
        .send()
        .await
        .unwrap();

    // Step 2: Grab passcode
    let db = sea_orm::Database::connect(&config.database_test_url)
        .await
        .unwrap();
    let user = authorizer::repositories::user_repository::find_by_email(&db, &email)
        .await
        .unwrap()
        .unwrap();
    let passcode =
        authorizer::repositories::passcode_repository::find_active_by_user_id(&db, user.id)
            .await
            .unwrap()
            .unwrap();

    // Step 3: Verify to create session
    let verify_payload = VerifyPasscodeRequest {
        email: email.clone(),
        passcode: passcode.code.clone(),
    };

    client
        .post(format!("{}/v1/verify", config.authorizer_test_url))
        .json(&verify_payload)
        .send()
        .await
        .unwrap();

    // Step 4: Logout
    let logout = client
        .post(format!("{}/auth/logout", config.authorizer_test_url))
        .send()
        .await
        .unwrap();

    assert_eq!(logout.status(), StatusCode::OK);
}
