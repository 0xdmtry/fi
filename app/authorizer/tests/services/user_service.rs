use authorizer::config::AppConfig;
use authorizer::models::{user, passcode};
use authorizer::repositories::{user_repository, passcode_repository};
use authorizer::services::user_service;
use sea_orm::{Database, DbConn, EntityTrait};
use serial_test::serial;
use uuid::Uuid;

fn new_email() -> String {
    format!("user-{}@example.com", Uuid::new_v4())
}

async fn setup_db() -> (DbConn, AppConfig) {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db = Database::connect(&config.database_test_url).await.unwrap();

    passcode::Entity::delete_many().exec(&db).await.unwrap();
    user::Entity::delete_many().exec(&db).await.unwrap();

    (db, config)
}

#[tokio::test]
#[serial]
async fn test_process_join_creates_user_and_passcode_and_sends_email() {
    let (db, config) = setup_db().await;
    let email = new_email();

    user_service::process_join_request(&db, &config, &email)
        .await
        .expect("join failed");

    // User should be created
    let user = user_repository::find_by_email(&db, &email)
        .await
        .expect("user lookup failed")
        .expect("user not found");

    // Passcode should exist for user
    let passcode = passcode_repository::find_active_by_user_id(&db, user.id)
        .await
        .expect("query failed")
        .expect("no active passcode found");

    assert_eq!(passcode.user_id, user.id);
    assert_eq!(passcode.used, false);
    assert_eq!(passcode.code.len() as u32, config.passcode_len);
}
