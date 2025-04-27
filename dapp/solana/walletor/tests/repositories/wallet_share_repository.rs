use chrono::Utc;
use sea_orm::{Database, DbConn, EntityTrait};
use uuid::Uuid;
use walletor::config::AppConfig;
use walletor::models::wallet_share;
use walletor::repositories::wallet_share_repository;

#[tokio::test]
async fn test_insert_and_find_wallet_share() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    wallet_share::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");

    let wallet_id = Uuid::new_v4();
    let encryption_secret_id = Uuid::new_v4();
    let encrypted_user_share = vec![1, 2, 3, 4];
    let server_share = vec![5, 6, 7, 8];

    let inserted = wallet_share_repository::insert_new_wallet_share(
        &db,
        wallet_id,
        encrypted_user_share.clone(),
        server_share.clone(),
        encryption_secret_id,
    )
    .await
    .expect("failed to insert wallet share");

    assert_eq!(inserted.wallet_id, wallet_id);
    assert_eq!(inserted.encryption_secret_id, encryption_secret_id);

    let found = wallet_share_repository::find_share_by_wallet_id(&db, wallet_id)
        .await
        .expect("failed to find wallet share");

    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.encrypted_user_share, encrypted_user_share);
    assert_eq!(found.server_share, server_share);
}
