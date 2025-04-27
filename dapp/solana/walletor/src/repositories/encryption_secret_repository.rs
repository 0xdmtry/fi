use crate::models::encryption_secret;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn find_secret_by_user_id(
    db: &DbConn,
    user_id: Uuid,
) -> anyhow::Result<Option<encryption_secret::Model>> {
    let result = encryption_secret::Entity::find()
        .filter(encryption_secret::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    Ok(result)
}

pub async fn insert_new_encryption_secret(
    db: &DbConn,
    user_id: Uuid,
    encrypted_secret: Vec<u8>,
    salt: Vec<u8>,
) -> anyhow::Result<encryption_secret::Model> {
    let new_secret = encryption_secret::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        encrypted_secret: Set(encrypted_secret),
        salt: Set(salt),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    Ok(new_secret.insert(db).await?)
}
