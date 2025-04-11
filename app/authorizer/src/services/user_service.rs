use sea_orm::DbConn;
use crate::repositories::user_repository;
use crate::repositories::passcode_repository;
use crate::services::email_client;

pub async fn process_join_request(email: &str, db: &DbConn) -> anyhow::Result<()> {
    let normalized_email = email.to_lowercase();

    let user = match user_repository::find_by_email(db, &normalized_email).await? {
        Some(u) => u,
        None => user_repository::insert_new_user(db, &normalized_email).await?,
    };

    let passcode = passcode_repository::generate_and_insert(db, &user).await?;

    email_client::send_passcode_email(&user.email, &passcode.code).await?;

    Ok(())
}