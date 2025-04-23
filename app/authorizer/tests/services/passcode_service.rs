use authorizer::config::AppConfig;
use authorizer::models::{passcode, user};
use authorizer::repositories::{passcode_repository, user_repository};
use authorizer::services::passcode_service;
use chrono::{Duration, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DbConn, EntityTrait, IntoActiveModel, QueryFilter, Set,
};
use serial_test::serial;
use uuid::Uuid;

fn new_test_email() -> String {
    format!("svc-test-{}@example.com", Uuid::new_v4())
}

async fn setup_db() -> (DbConn, AppConfig) {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db = Database::connect(&config.database_test_url)
        .await
        .expect("DB connect failed");

    passcode::Entity::delete_many().exec(&db).await.unwrap();
    user::Entity::delete_many().exec(&db).await.unwrap();

    (db, config)
}

#[tokio::test]
#[serial]
async fn test_get_or_create_creates_new_if_none_exists() {
    let (db, config) = setup_db().await;
    let user = user_repository::insert_new_user(&db, &new_test_email())
        .await
        .unwrap();

    let pass = passcode_service::get_or_create_passcode(&db, &config, &user)
        .await
        .expect("failed to create");

    assert_eq!(pass.user_id, user.id);
    assert_eq!(pass.used, false);
    assert_eq!(pass.code.len() as u32, config.passcode_len);
    assert!(pass.expired_at > Utc::now());
}

#[tokio::test]
#[serial]
async fn test_get_or_create_reuses_existing_passcode() {
    let (db, config) = setup_db().await;
    let user = user_repository::insert_new_user(&db, &new_test_email())
        .await
        .unwrap();

    // First insert
    let first = passcode_service::get_or_create_passcode(&db, &config, &user)
        .await
        .expect("failed to create");

    // Try again
    let second = passcode_service::get_or_create_passcode(&db, &config, &user)
        .await
        .expect("failed to reuse");

    assert_eq!(first.id, second.id, "should reuse existing valid passcode");
}

#[tokio::test]
#[serial]
async fn test_verify_passcode_succeeds() {
    let (db, config) = setup_db().await;

    let email = format!("verify-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    let pass = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap();

    let result = passcode_service::verify_passcode(&db, &config, &email, &pass.code).await;

    assert!(result.is_ok());

    let all = passcode::Entity::find()
        .filter(passcode::Column::UserId.eq(user.id))
        .all(&db)
        .await
        .unwrap();
    assert!(all.iter().all(|p| p.used));
}

#[tokio::test]
#[serial]
async fn test_verify_passcode_fails_if_user_not_found() {
    let (db, config) = setup_db().await;
    let email = format!("ghost-{}@example.com", Uuid::new_v4());
    let result = passcode_service::verify_passcode(&db, &config, &email, "1234").await;

    assert!(result.is_err());

    assert_eq!(result.unwrap_err().to_string(), "User not found");
}

#[tokio::test]
#[serial]
async fn test_verify_passcode_fails_if_code_is_wrong() {
    let (db, config) = setup_db().await;

    let email = format!("verify-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap();

    let result = passcode_service::verify_passcode(&db, &config, &email, "wrong-code").await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Invalid passcode");
}

#[tokio::test]
#[serial]
async fn test_verify_passcode_fails_if_expired() {
    let (db, config) = setup_db().await;

    let email = format!("verify-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    let expired = passcode::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        code: Set("1234".into()),
        attempt_count: Set(0),
        resend_count: Set(0),
        used: Set(false),
        expired_at: Set(Utc::now() - Duration::seconds(10)),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    expired.insert(&db).await.unwrap();

    let result = passcode_service::verify_passcode(&db, &config, &email, "1234").await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "No active passcode found");
}

#[tokio::test]
#[serial]
async fn test_verify_passcode_fails_after_max_attempts() {
    let (db, mut config) = setup_db().await;

    config.passcode_max_attempts = 3;

    let email = format!("verify-{}@example.com", Uuid::new_v4());
    let user = user_repository::insert_new_user(&db, &email).await.unwrap();

    let mut pass = passcode_repository::generate_and_insert(&db, &config, &user)
        .await
        .unwrap()
        .into_active_model();

    pass.attempt_count = Set(2);
    pass.update(&db).await.unwrap();

    let result = passcode_service::verify_passcode(&db, &config, &email, "wrong").await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Too many incorrect attempts"
    );
}
