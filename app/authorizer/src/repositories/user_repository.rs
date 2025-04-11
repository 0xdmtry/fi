use sea_orm::{DbConn, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use uuid::Uuid;
use chrono::Utc;
use crate::models::user;

pub async fn find_by_email(db: &DbConn, email: &str) -> anyhow::Result<Option<user::Model>> {
    let result = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?;

    Ok(result)
}

pub async fn insert_new_user(db: &DbConn, email: &str) -> anyhow::Result<user::Model> {
    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(email.to_owned()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        deleted_at: Set(None),
    };

    Ok(new_user.insert(db).await?)
}