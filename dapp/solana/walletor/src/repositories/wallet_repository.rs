use crate::models::wallet;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn find_wallet_by_user_id(
    db: &DbConn,
    user_id: Uuid,
) -> anyhow::Result<Option<wallet::Model>> {
    let result = wallet::Entity::find()
        .filter(wallet::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    Ok(result)
}

pub async fn find_wallet_by_wallet_address(
    db: &DbConn,
    wallet_address: &str,
) -> anyhow::Result<Option<wallet::Model>> {
    let result = wallet::Entity::find()
        .filter(wallet::Column::WalletAddress.eq(wallet_address))
        .one(db)
        .await?;

    Ok(result)
}

pub async fn insert_new_wallet(
    db: &DbConn,
    user_id: Uuid,
    wallet_address: &str,
    wallet_public_key: &str,
) -> anyhow::Result<wallet::Model> {
    let new_wallet = wallet::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        wallet_address: Set(wallet_address.to_owned()),
        wallet_public_key: Set(wallet_public_key.to_owned()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    Ok(new_wallet.insert(db).await?)
}

pub async fn find_wallet_by_id(
    db: &DbConn,
    wallet_id: Uuid,
) -> anyhow::Result<Option<wallet::Model>> {
    let result = wallet::Entity::find()
        .filter(wallet::Column::Id.eq(wallet_id))
        .one(db)
        .await?;

    Ok(result)
}
