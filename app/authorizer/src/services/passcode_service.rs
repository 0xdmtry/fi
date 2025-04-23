use crate::config::AppConfig;
use crate::models::{passcode, user};
use crate::repositories::{passcode_repository, user_repository};
use crate::utils::normalize::normalize_email;
use anyhow::{Result, anyhow};
use sea_orm::{DbConn, EntityTrait};

pub async fn get_or_create_passcode(
    db: &DbConn,
    config: &AppConfig,
    user: &user::Model,
) -> anyhow::Result<passcode::Model> {
    match passcode_repository::find_active_by_user_id(db, user.id).await? {
        Some(active) => Ok(active),
        None => passcode_repository::generate_and_insert(db, config, user).await,
    }
}

pub async fn verify_passcode(
    db: &DbConn,
    config: &AppConfig,
    email: &str,
    input_code: &str,
) -> Result<()> {
    let normalized = normalize_email(email);

    let user = user_repository::find_by_email(db, &normalized)
        .await?
        .ok_or_else(|| anyhow!("User not found"))?;

    if let Some(pass) =
        passcode_repository::find_matching_active_passcode(db, user.id, input_code).await?
    {
        passcode_repository::mark_all_active_codes_used(db, user.id).await?;
        return Ok(());
    }

    if let Some(active_code) = passcode_repository::find_active_by_user_id(db, user.id).await? {
        passcode_repository::increment_attempt_count(db, active_code.id).await?;

        if active_code.attempt_count + 1 >= config.passcode_max_attempts as i32 {
            return Err(anyhow!("Too many incorrect attempts"));
        }

        return Err(anyhow!("Invalid passcode"));
    }

    Err(anyhow!("No active passcode found"))
}
