use sea_orm::{DbConn, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, ActiveModelTrait, Set};
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::models::passcode;
use crate::models::user;
use rand::Rng;
use crate::config::AppConfig;

pub async fn find_active_by_user_id(db: &DbConn, user_id: Uuid) -> anyhow::Result<Option<passcode::Model>> {
    let now = Utc::now();

    let active = passcode::Entity::find()
                    .filter(passcode::Column::UserId.eq(user_id))
                    .filter(passcode::Column::Used.eq(false))
                    .filter(passcode::Column::ExpiredAt.gt(now))
                    .order_by_desc(passcode::Column::CreatedAt)
                    .one(db)
                    .await?;

    Ok(active)
}

pub async fn generate_and_insert(db: &DbConn, config: &AppConfig, user: &user::Model) -> anyhow::Result<passcode::Model> {
    let passcode = {
        let mut rng = rand::rng();
        format!("{:04}", rng.random_range(config.passcode_min_range..config.passcode_max_range))
    };
    let expires = Utc::now() + Duration::seconds(config.passcode_ttl_seconds);

    let new_passcode = passcode::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        code: Set(passcode.clone()),
        attempt_count: Set(0),
        resend_count: Set(0),
        used: Set(false),
        expired_at: Set(expires),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    Ok(new_passcode.insert(db).await?)
}