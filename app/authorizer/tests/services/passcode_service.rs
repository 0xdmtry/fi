use authorizer::config::AppConfig;
use authorizer::models::{passcode, user};
use authorizer::repositories::user_repository;
use authorizer::services::passcode_service;
use chrono::Utc;
use sea_orm::{Database, DbConn, EntityTrait};
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
    let user = user_repository::insert_new_user(&db, &new_test_email()).await.unwrap();

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
    let user = user_repository::insert_new_user(&db, &new_test_email()).await.unwrap();

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
