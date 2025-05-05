use chrono::{Duration, Utc};
use sea_orm::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DbConn, EntityTrait, IntoActiveModel, QueryFilter, Set,
};
use serial_test::serial;
use transactor::config::AppConfig;
use transactor::models::tx;
use transactor::repositories::tx_repository;
use uuid::Uuid;

use sea_orm::entity::prelude::*;
use serde_json::json;

use transactor::repositories::tx_repository::{find_by_signature, insert_tx};

#[tokio::test]
async fn test_insert_and_get_tx() {
    let config = AppConfig::from_env_with_custom_file(".test.env");
    let db: DbConn = Database::connect(&config.database_test_url).await.unwrap();

    let signature = "test_sig_123";
    let user_id = Uuid::new_v4();

    let new_tx = tx::ActiveModel {
        signature: Set(signature.to_string()),
        user_id: Set(user_id),
        wallet_id: Set(None),
        slot: Set(12345678),
        block_time: Set(Some(1_700_000_000)),
        success: Set(true),
        fee_payer: Set("payer111...".into()),
        signer_addresses: Set(json!(["signer1", "signer2"])),
        instructions: Set(Some(json!([{ "program": "spl_token" }]))),
        log_messages: Set(None),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };

    let inserted = insert_tx(&db, new_tx).await.unwrap();
    assert_eq!(inserted.signature, signature);

    let found = find_by_signature(&db, signature).await.unwrap();
    assert!(found.is_some());
}
