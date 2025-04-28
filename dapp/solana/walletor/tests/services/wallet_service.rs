use sea_orm::{ColumnTrait, Database, DbConn, EntityTrait, QueryFilter};
use uuid::Uuid;
use walletor::config::AppConfig;
use walletor::models::{encryption_secret, wallet, wallet_share};
use walletor::services::wallet_service;

#[tokio::test]
async fn test_create_wallet_service() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    // Cleanup before test
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

    let user_id = Uuid::new_v4();
    let passcode = "1234";

    let result = wallet_service::create_wallet_service(&db, &config, user_id, passcode)
        .await
        .expect("failed to create wallet");

    // Validate wallet is created
    let found_wallet = wallet::Entity::find_by_id(result.wallet_id)
        .one(&db)
        .await
        .expect("failed to query wallet")
        .expect("wallet not found");

    assert_eq!(found_wallet.wallet_address, result.wallet_address);
    assert_eq!(found_wallet.wallet_public_key, result.wallet_public_key);

    // Validate wallet share
    let found_share = wallet_share::Entity::find()
        .filter(wallet_share::Column::WalletId.eq(result.wallet_id))
        .one(&db)
        .await
        .expect("failed to query wallet_share");

    assert!(found_share.is_some(), "wallet share not found");

    // Validate encryption secret
    let found_secret = encryption_secret::Entity::find()
        .filter(encryption_secret::Column::UserId.eq(user_id))
        .one(&db)
        .await
        .expect("failed to query encryption_secret");

    assert!(found_secret.is_some(), "encryption secret not found");
}
