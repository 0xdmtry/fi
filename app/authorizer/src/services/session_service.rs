use crate::config::AppConfig;
use crate::models::{session, user};
use crate::repositories::session_repository;
use crate::utils::token::generate_jwt;
use anyhow::{Result, anyhow};
use chrono::{Duration, Utc};
use sea_orm::{DbConn, EntityTrait};
use uuid::Uuid;

pub async fn create_session(
    db: &DbConn,
    config: &AppConfig,
    user: &user::Model,
    user_agent: &str,
    ip_address: &str,
) -> Result<(String, Uuid)> {
    let expires_at = Utc::now() + Duration::seconds(config.session_ttl_seconds);

    let session =
        session_repository::create_session(db, user.id, user_agent, ip_address, expires_at).await?;

    let access_token = generate_jwt(config, user)?;
    let refresh_token = session.id;

    Ok((access_token, refresh_token))
}

pub async fn validate_and_refresh(
    db: &DbConn,
    config: &AppConfig,
    session_id: Uuid,
) -> Result<String> {
    let session = session_repository::find_active_session(db, session_id)
        .await?
        .ok_or_else(|| anyhow!("Session not found or expired"))?;

    let user = user::Entity::find_by_id(session.user_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow!("User not found"))?;

    let access_token = generate_jwt(config, &user)?;

    Ok(access_token)
}

pub async fn revoke_session(db: &DbConn, user_id: Uuid) -> Result<()> {
    session_repository::revoke_all_user_sessions(db, user_id).await
}

pub async fn revoke_all_sessions(db: &DbConn, user_id: Uuid) -> Result<()> {
    session_repository::revoke_all_user_sessions(db, user_id).await
}
