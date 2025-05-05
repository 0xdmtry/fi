use crate::models::tx;
use sea_orm::*;
use uuid::Uuid;

pub async fn insert_tx(db: &DbConn, new_tx: tx::ActiveModel) -> Result<tx::Model, DbErr> {
    new_tx.insert(db).await
}

pub async fn find_by_signature(db: &DbConn, signature: &str) -> Result<Option<tx::Model>, DbErr> {
    tx::Entity::find_by_id(signature.to_string()).one(db).await
}

pub async fn find_by_user_id(db: &DbConn, user_id: Uuid) -> Result<Vec<tx::Model>, DbErr> {
    tx::Entity::find()
        .filter(tx::Column::UserId.eq(user_id))
        .order_by_desc(tx::Column::CreatedAt)
        .all(db)
        .await
}
