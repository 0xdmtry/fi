use chrono::Utc;
use sea_orm::{Database, DbConn, EntityTrait};
use uuid::Uuid;
use walletor::config::AppConfig;
use walletor::models::encryption_secret;
use walletor::repositories::encryption_secret_repository;

#[tokio::test]
async fn test_insert_and_find_encryption_secret() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    encryption_secret::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");

    let user_id = Uuid::new_v4();
    let encrypted_secret = vec![10, 20, 30, 40];
    let salt = vec![50, 60, 70, 80];

    let inserted = encryption_secret_repository::insert_new_encryption_secret(
        &db,
        user_id,
        encrypted_secret.clone(),
        salt.clone(),
    )
    .await
    .expect("failed to insert encryption secret");

    assert_eq!(inserted.user_id, user_id);
    assert_eq!(inserted.encrypted_secret, encrypted_secret);

    let found = encryption_secret_repository::find_secret_by_user_id(&db, user_id)
        .await
        .expect("failed to find encryption secret");

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, inserted.id);
}
