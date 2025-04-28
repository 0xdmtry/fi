use walletor::config::AppConfig;
use walletor::models::wallet;
use walletor::repositories::wallet_repository;
use sea_orm::{Database, DbConn, EntityTrait};
use uuid::Uuid;

#[tokio::test]
async fn test_insert_wallet_success() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    // Cleanup
    wallet::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");

    // Insert
    let user_id = Uuid::new_v4();
    let wallet_address = format!("wallet-{}", Uuid::new_v4());
    let wallet_public_key = format!("public-{}", Uuid::new_v4());

    let inserted = wallet_repository::insert_new_wallet(&db, user_id, &wallet_address, &wallet_public_key)
        .await
        .expect("failed to insert wallet");

    assert_eq!(inserted.user_id, user_id);
    assert_eq!(inserted.wallet_address, wallet_address);
    assert_eq!(inserted.wallet_public_key, wallet_public_key);
}

#[tokio::test]
async fn test_insert_and_find_wallet_success() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url)
        .await
        .expect("failed to connect");

    // Cleanup once at the beginning
    wallet::Entity::delete_many()
        .exec(&db)
        .await
        .expect("cleanup failed");

    // Insert
    let user_id = Uuid::new_v4();
    let wallet_address = format!("wallet-{}", Uuid::new_v4());
    let wallet_public_key = format!("public-{}", Uuid::new_v4());

    let inserted = wallet_repository::insert_new_wallet(&db, user_id, &wallet_address, &wallet_public_key)
        .await
        .expect("failed to insert wallet");

    // Find by user_id
    let found_by_user = wallet_repository::find_wallet_by_user_id(&db, user_id)
        .await
        .expect("failed to find wallet by user_id");

    assert!(found_by_user.is_some());
    let found_by_user = found_by_user.unwrap();
    assert_eq!(found_by_user.id, inserted.id);

    // Find by wallet_address
    let found_by_address = wallet_repository::find_wallet_by_wallet_address(&db, &wallet_address)
        .await
        .expect("failed to find wallet by address");

    assert!(found_by_address.is_some());
    let found_by_address = found_by_address.unwrap();
    assert_eq!(found_by_address.id, inserted.id);
}
