use sea_orm::{DbConn, EntityTrait, ActiveModelTrait, ColumnTrait, QueryFilter, Set, QueryOrder};
use uuid::Uuid;
use chrono::{Utc, Duration};
use rand::Rng;
use crate::models::{user, passcode};

const MAX_ATTEMPTS: i32 = 5;
const MAX_RESENDS: i32 = 3;
const CODE_TTL_MINUTES: i64 = 5;

pub async fn get_or_create_passcode(
    db: &DbConn,
    user: &user::Model
) -> anyhow::Result<passcode::Model> {
    let now = Utc::now();

    let existing = passcode::Entity::find()
                    .filter(passcode::Column::UserId.eq(user.id))
                    .filter(passcode::Column::Used.eq(false))
                    .filter(passcode::Column::ExpiredAt.gt(now))
                    .order_by_desc(passcode::Column::CreatedAt)
                    .one(db)
                    .await?;
    
    let new_code = format!("{:04}", rand::rng().gen_range(1000..10000));
    let expires = now + Duration::minutes(CODE_TTL_MINUTES);

    let new_passcode = passcode::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        code: Set(new_code),
        attempt_count: Set(0),
        resend_count: Set(0),
        used: Set(false),
        expired_at: Set(expires),
        created_at: Set(now),
        updated_at: Set(now),
    };

    Ok(new_passcode.insert(db).await?)
}

