use crate::config::AppConfig;
use crate::repositories::email_repository::SaveEmailArgs;
use anyhow::Result;
use sea_orm::DatabaseConnection;

pub trait Emailer: Send + Sync {
    fn save_email(
        &self,
        config: &AppConfig,
        db_conn: &DatabaseConnection,
        args: SaveEmailArgs,
    ) -> Result<()>;

    // Passcode
    fn send_and_save_passcode_email(
        &self,
        config: &AppConfig,
        db_conn: &DatabaseConnection,
        email: &str,
        passcode: &str,
    ) -> Result<()>;
    fn send_passcode_email(&self, config: &AppConfig, email: &str, passcode: &str) -> Result<()>;

    // Success Passcode
    fn send_and_save_success_passcode_email(
        &self,
        config: &AppConfig,
        db_conn: &DatabaseConnection,
        email: &str,
    ) -> Result<()>;
    fn send_success_passcode_email(&self, config: &AppConfig, email: &str) -> Result<()>;

    // Failed Passcode
    fn send_and_save_failed_passcode_email(
        &self,
        config: &AppConfig,
        db_conn: &DatabaseConnection,
        email: &str,
    ) -> Result<()>;
    fn send_failed_passcode_email(&self, config: &AppConfig, email: &str) -> Result<()>;
}
