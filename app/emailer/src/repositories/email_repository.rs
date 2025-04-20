use anyhow::Result;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DbConn, Set};
use uuid::Uuid;

use crate::models::{EmailType, Provider, email};

#[derive(Clone)]
pub struct SaveEmailArgs {
    pub recipient: String,
    pub email_type: EmailType,

    pub subject: Option<String>,
    pub content: Option<String>,

    pub template_type: Option<String>,
    pub template_language: Option<String>,
    pub rendered_with: Option<String>,

    pub provider: Provider,
    pub status: String,
    pub error_message: Option<String>,
    pub message_id: Option<String>,

    pub retry_count: i32,
    pub sent_by_fallback: bool,
    pub opened_at: Option<chrono::DateTime<Utc>>,
}

pub async fn save_sent_email(db: &DbConn, args: SaveEmailArgs) -> Result<email::Model> {
    let now = Utc::now();

    let new_email = email::ActiveModel {
        id: Set(Uuid::new_v4()),
        recipient: Set(args.recipient),
        email_type: Set(args.email_type),

        subject: Set(args.subject),
        content: Set(args.content),

        template_type: Set(args.template_type),
        template_language: Set(args.template_language),
        rendered_with: Set(args.rendered_with),

        provider: Set(args.provider),
        status: Set(args.status),
        error_message: Set(args.error_message),
        message_id: Set(args.message_id),

        retry_count: Set(args.retry_count),
        sent_by_fallback: Set(args.sent_by_fallback),
        opened_at: Set(args.opened_at),

        created_at: Set(now),
        updated_at: Set(now),
    };

    Ok(new_email.insert(db).await?)
}
