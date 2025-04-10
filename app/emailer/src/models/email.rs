use sea_orm::entity::prelude::*;
use uuid::Uuid;
use crate::models::{EmailType, Provider};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "emails")]
pub struct Model {

    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub recipient: String,

    pub subject: Option<String>,
    pub email_type: EmailType,
    pub template_type: Option<String>,
    pub template_language: Option<String>,
    pub rendered_with: Option<String>,

    pub provider: Provider,
    pub status: String,
    pub error_message: Option<String>,
    pub message_id: Option<String>,

    pub retry_count: i32,
    pub sent_by_fallback: bool,

    pub opened_at: Option<DateTimeUtc>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}