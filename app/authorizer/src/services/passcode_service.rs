use sea_orm::{DbConn, EntityTrait};
use crate::models::{user, passcode};
use crate::repositories::passcode_repository;
use crate::config::AppConfig;


pub async fn get_or_create_passcode(
    db: &DbConn,
    config: &AppConfig,
    user: &user::Model
) -> anyhow::Result<passcode::Model> {

    match passcode_repository::find_active_by_user_id(db, user.id).await? {
        Some(active) => Ok(active),
        None => passcode_repository::generate_and_insert(db, config, user).await
    }
}