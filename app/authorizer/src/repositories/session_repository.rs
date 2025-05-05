use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, IntoActiveModel, QueryFilter, Set,
};
use uuid::Uuid;

use crate::models::session;

pub async fn create_session(
    db: &DbConn,
    user_id: Uuid,
    user_agent: &str,
    ip_address: &str,
    expires_at: DateTime<Utc>,
) -> anyhow::Result<session::Model> {
    let new_session = session::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        user_agent: Set(user_agent.to_string()),
        ip_address: Set(ip_address.to_string()),
        expires_at: Set(expires_at),
        revoked_at: Set(None),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    Ok(new_session.insert(db).await?)
}

pub async fn find_active_session(
    db: &DbConn,
    session_id: Uuid,
) -> anyhow::Result<Option<session::Model>> {
    let now = Utc::now();

    let result = session::Entity::find()
        .filter(session::Column::Id.eq(session_id))
        .filter(session::Column::RevokedAt.is_null())
        .filter(session::Column::ExpiresAt.gt(now))
        .one(db)
        .await?;

    Ok(result)
}

pub async fn revoke_session(db: &DbConn, session_id: Uuid) -> anyhow::Result<()> {
    let session = session::Entity::find_by_id(session_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    let mut model = session.into_active_model();
    model.revoked_at = Set(Some(Utc::now()));
    model.updated_at = Set(Utc::now());

    model.update(db).await?;

    Ok(())
}

pub async fn revoke_all_user_sessions(db: &DbConn, user_id: Uuid) -> anyhow::Result<()> {
    let sessions = session::Entity::find()
        .filter(session::Column::UserId.eq(user_id))
        .filter(session::Column::RevokedAt.is_null())
        .all(db)
        .await?;

    for s in sessions {
        let mut active = s.into_active_model();
        active.revoked_at = Set(Some(Utc::now()));
        active.updated_at = Set(Utc::now());
        active.update(db).await?;
    }

    Ok(())
}
