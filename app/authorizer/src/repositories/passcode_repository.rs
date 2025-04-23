use crate::config::AppConfig;
use crate::models::passcode;
use crate::models::user;
use chrono::{Duration, Utc};
use rand::Rng;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

pub async fn find_matching_active_passcode(
    db: &DbConn,
    user_id: Uuid,
    code: &str,
) -> anyhow::Result<Option<passcode::Model>> {
    let now = Utc::now();

    let found = passcode::Entity::find()
        .filter(passcode::Column::UserId.eq(user_id))
        .filter(passcode::Column::Code.eq(code))
        .filter(passcode::Column::Used.eq(false))
        .filter(passcode::Column::ExpiredAt.gt(now))
        .order_by_desc(passcode::Column::CreatedAt)
        .one(db)
        .await?;

    Ok(found)
}

pub async fn increment_attempt_count(db: &DbConn, code_id: Uuid) -> anyhow::Result<()> {
    let model = passcode::Entity::find_by_id(code_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Passcode not found"))?;

    let new_attempt_count = model.attempt_count + 1;

    let mut active = model.into_active_model();
    active.attempt_count = Set(new_attempt_count);
    active.updated_at = Set(Utc::now());

    active.update(db).await?;

    Ok(())
}

pub async fn mark_all_active_codes_used(db: &DbConn, user_id: Uuid) -> anyhow::Result<()> {
    let now = Utc::now();

    let active_codes = passcode::Entity::find()
        .filter(passcode::Column::UserId.eq(user_id))
        .filter(passcode::Column::Used.eq(false))
        .filter(passcode::Column::ExpiredAt.gt(now))
        .all(db)
        .await?;

    for code in active_codes {
        let mut model = code.into_active_model();
        model.used = Set(true);
        model.updated_at = Set(Utc::now());
        model.update(db).await?;
    }

    Ok(())
}

pub async fn find_active_by_user_id(
    db: &DbConn,
    user_id: Uuid,
) -> anyhow::Result<Option<passcode::Model>> {
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

pub async fn generate_and_insert(
    db: &DbConn,
    config: &AppConfig,
    user: &user::Model,
) -> anyhow::Result<passcode::Model> {
    let passcode = {
        let mut rng = rand::rng();
        let number = rng.random_range(config.passcode_min_range..config.passcode_max_range);
        format!("{:0width$}", number, width = config.passcode_len as usize)
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
