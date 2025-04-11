use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Email::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Email::Id).uuid().not_null().primary_key())

                    .col(ColumnDef::new(Email::Recipient).string_len(254).not_null())
                    .col(ColumnDef::new(Email::Subject).string().null())

                    .col(ColumnDef::new(Email::EmailType).string_len(32).not_null())
                    .col(ColumnDef::new(Email::TemplateType).string().null())
                    .col(ColumnDef::new(Email::TemplateLanguage).string().null())
                    .col(ColumnDef::new(Email::RenderedWith).string().null())

                    .col(ColumnDef::new(Email::Provider).string_len(32).not_null())
                    .col(ColumnDef::new(Email::Status).string_len(32).not_null())
                    .col(ColumnDef::new(Email::ErrorMessage).text().null())
                    .col(ColumnDef::new(Email::MessageId).string().null())

                    .col(ColumnDef::new(Email::RetryCount).integer().not_null())
                    .col(ColumnDef::new(Email::SentByFallback).boolean().not_null())

                    .col(ColumnDef::new(Email::OpenedAt).timestamp().null())
                    .col(ColumnDef::new(Email::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Email::UpdatedAt).timestamp().not_null())

                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Email::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Email {
    Table,
    Id,

    Recipient,
    Subject,
    EmailType,
    TemplateType,
    TemplateLanguage,
    RenderedWith,

    Provider,
    Status,
    ErrorMessage,
    MessageId,

    RetryCount,
    SentByFallback,

    OpenedAt,
    CreatedAt,
    UpdatedAt,
}