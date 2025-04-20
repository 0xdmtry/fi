use anyhow::{Result, anyhow};
use chrono::Utc;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::config::AppConfig;
use crate::models::{EmailType, Provider};
use crate::repositories::email_repository::{SaveEmailArgs, save_sent_email};
use crate::services::emailer_service::Emailer;

pub struct MailhogEmailer;

impl MailhogEmailer {
    pub fn new() -> Self {
        Self
    }
}
impl Emailer for MailhogEmailer {
    fn send_passcode_email(&self, email: &str, passcode: &str) -> Result<()> {
        // Simulated send
        println!("[MAILHOG] Sending passcode {} to {}", passcode, email);
        Ok(())
    }

    fn save_email(
        &self,
        config: &AppConfig,
        db_conn: &DatabaseConnection,
        args: SaveEmailArgs,
    ) -> Result<()> {
        let db = db_conn.clone();

        // Save in background to avoid blocking
        tokio::spawn(async move {
            if let Err(e) = save_sent_email(&db, args).await {
                eprintln!("⚠️ Failed to save email record: {e}");
            }
        });

        Ok(())
    }

    fn send_and_save_passcode_email(
        &self,
        config: &AppConfig,
        db_conn: &DatabaseConnection,
        email: &str,
        passcode: &str,
    ) -> Result<()> {
        self.send_passcode_email(email, passcode)?;

        let args = SaveEmailArgs {
            recipient: email.to_string(),
            email_type: EmailType::Passcode,
            subject: Some("Your login code".into()),
            content: Some(format!("Your passcode is: {passcode}")),
            template_type: None,
            template_language: None,
            rendered_with: None,
            provider: Provider::Mailhog,
            status: "sent".into(),
            error_message: None,
            message_id: None,
            retry_count: 0,
            sent_by_fallback: false,
            opened_at: None,
        };

        // Persist
        let db = db_conn.clone();
        tokio::spawn(async move {
            if let Err(e) = save_sent_email(&db, args).await {
                eprintln!("⚠️ Failed to save email record: {e}");
            }
        });

        Ok(())
    }
}
