use crate::config::AppConfig;
use crate::repositories::user_repository;
use crate::services::email_client;
use crate::services::passcode_service;
use crate::services::walletor_client;
use sea_orm::DbConn;

pub async fn process_join_request(
    db: &DbConn,
    config: &AppConfig,
    email: &str,
) -> anyhow::Result<()> {
    let normalized_email = email.to_lowercase();

    // Check if user already exists
    let existing_user = user_repository::find_by_email(db, &normalized_email).await?;

    let (user, is_new_user) = match existing_user {
        Some(u) => (u, false),
        None => {
            let new_user = user_repository::insert_new_user(db, &normalized_email).await?;
            (new_user, true)
        }
    };

    let passcode = passcode_service::get_or_create_passcode(db, config, &user).await?;

    email_client::send_passcode_email(config, &user.email, &passcode.code).await?;

    // Only call Walletor if user is new
    if is_new_user {
        walletor_client::send_walletor_signup_request(config, user.id, &passcode.code).await?;
    }

    Ok(())
}
