use sea_orm::DbConn;
use crate::repositories::user_repository;
use crate::services::email_client;
use crate::services::passcode_service;
use crate::config::AppConfig;

pub async fn process_join_request(db: &DbConn, config: &AppConfig, email: &str) -> anyhow::Result<()> {
    let normalized_email = email.to_lowercase();

    let user = match user_repository::find_by_email(db, &normalized_email).await? {
        Some(u) => u,
        None => user_repository::insert_new_user(db, &normalized_email).await?,
    };

    let passcode = passcode_service::get_or_create_passcode(db, config, &user).await?;

    email_client::send_passcode_email(config, &user.email, &passcode.code).await?;

    Ok(())
}