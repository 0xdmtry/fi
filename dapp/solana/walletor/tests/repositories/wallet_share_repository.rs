use walletor::config::AppConfig;
use walletor::models::{wallet, wallet_share, encryption_secret};
use walletor::repositories::{wallet_repository, wallet_share_repository, encryption_secret_repository};
use sea_orm::{Database, DbConn, EntityTrait};
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_insert_and_find_wallet_share() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    // Clean tables
    wallet_share::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup wallet_share failed");

    encryption_secret::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup encryption_secret failed");

    wallet::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup wallet failed");

    // First insert a wallet
    let user_id = Uuid::new_v4();
    let wallet_address = format!("wallet-{}", Uuid::new_v4());
    let wallet_public_key = format!("public-{}", Uuid::new_v4());

    let inserted_wallet = wallet_repository::insert_new_wallet(&db, user_id, &wallet_address, &wallet_public_key)
        .await
        .expect("failed to insert wallet");

    // Insert an encryption_secret because wallet_shares FK depends on it
    let encrypted_secret = vec![10, 20, 30, 40];
    let salt = vec![50, 60, 70, 80];

    let inserted_secret = encryption_secret_repository::insert_new_encryption_secret(
        &db,
        user_id,
        encrypted_secret.clone(),
        salt.clone(),
    )
        .await
        .expect("failed to insert encryption secret");

    // Now insert wallet_share
    let encrypted_user_share = vec![1, 2, 3, 4];
    let server_share = vec![5, 6, 7, 8];

    let inserted_share = wallet_share_repository::insert_new_wallet_share(
        &db,
        inserted_wallet.id,
        encrypted_user_share.clone(),
        server_share.clone(),
        inserted_secret.id, // must pass valid encryption_secret_id
    )
        .await
        .expect("failed to insert wallet share");

    // Validate
    assert_eq!(inserted_share.wallet_id, inserted_wallet.id);
    assert_eq!(inserted_share.encryption_secret_id, inserted_secret.id);

    let found = wallet_share_repository::find_share_by_wallet_id(&db, inserted_wallet.id)
        .await
        .expect("failed to find wallet share");

    assert!(found.is_some(), "wallet share not found");
    let found = found.unwrap();

    assert_eq!(found.encrypted_user_share, encrypted_user_share);
    assert_eq!(found.server_share, server_share);
}
