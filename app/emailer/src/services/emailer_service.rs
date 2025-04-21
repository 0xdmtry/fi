use crate::config::AppConfig;
use crate::repositories::email_repository::SaveEmailArgs;
use anyhow::Result;
use sea_orm::DatabaseConnection;

pub trait Emailer: Send + Sync {
    fn send_passcode_email(&self, config: &AppConfig, email: &str, passcode: &str) -> Result<()>;
    fn save_email(
        &self,
        config: &AppConfig,
        db_conn: &DatabaseConnection,
        args: SaveEmailArgs,
    ) -> Result<()>;

    fn send_and_save_passcode_email(
        &self,
        config: &AppConfig,
        db_conn: &DatabaseConnection,
        email: &str,
        passcode: &str,
    ) -> Result<()>;
}
