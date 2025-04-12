use sea_orm::{DbConn, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::models::passcode;
use crate::models::user;
use rand::Rng;

pub async fn generate_and_insert(db: &DbConn, user: &user::Model) -> anyhow::Result<passcode::Model> {
    let mut rng = rand::rng();
    let passcode = format!("{:04}", rng.gen_range(1000..10000));
    let expires = Utc::now() + Duration::minutes(5);

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