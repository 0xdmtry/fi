use chrono::Utc;
use sea_orm::{Database, DbConn, EntityTrait};
use uuid::Uuid;
use walletor::config::AppConfig;
use walletor::models::wallet;
use walletor::repositories::wallet_repository;

#[tokio::test]
async fn test_insert_and_find_wallet() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    wallet::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");

    let user_id = Uuid::new_v4();
    let wallet_address = format!("WalletAddress{}", Uuid::new_v4());
    let wallet_public_key = format!("WalletPublicKey{}", Uuid::new_v4());

    let inserted =
        wallet_repository::insert_new_wallet(&db, user_id, &wallet_address, &wallet_public_key)
            .await
            .expect("failed to insert wallet");

    assert_eq!(inserted.user_id, user_id);
    assert_eq!(inserted.wallet_address, wallet_address);

    let found = wallet_repository::find_wallet_by_user_id(&db, user_id)
        .await
        .expect("failed to find wallet");

    assert!(found.is_some());
    assert_eq!(found.unwrap().id, inserted.id);

    let found_by_address = wallet_repository::find_wallet_by_wallet_address(&db, &wallet_address)
        .await
        .expect("failed to find wallet by address");

    assert!(found_by_address.is_some());
    assert_eq!(found_by_address.unwrap().id, inserted.id);
}
