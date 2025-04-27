use crate::models::wallet_share;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn find_share_by_wallet_id(
    db: &DbConn,
    wallet_id: Uuid,
) -> anyhow::Result<Option<wallet_share::Model>> {
    let result = wallet_share::Entity::find()
        .filter(wallet_share::Column::WalletId.eq(wallet_id))
        .one(db)
        .await?;

    Ok(result)
}

pub async fn insert_new_wallet_share(
    db: &DbConn,
    wallet_id: Uuid,
    encrypted_user_share: Vec<u8>,
    server_share: Vec<u8>,
    encryption_secret_id: Uuid,
) -> anyhow::Result<wallet_share::Model> {
    let new_share = wallet_share::ActiveModel {
        id: Set(Uuid::new_v4()),
        wallet_id: Set(wallet_id),
        encrypted_user_share: Set(encrypted_user_share),
        server_share: Set(server_share),
        encryption_secret_id: Set(encryption_secret_id),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    Ok(new_share.insert(db).await?)
}
