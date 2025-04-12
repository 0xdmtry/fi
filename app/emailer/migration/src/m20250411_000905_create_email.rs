use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Emails::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Emails::Id).uuid().not_null().primary_key())

                    .col(ColumnDef::new(Emails::Recipient).string_len(254).not_null())
                    .col(ColumnDef::new(Emails::Subject).string().null())

                    .col(ColumnDef::new(Emails::EmailType).string_len(32).not_null())
                    .col(ColumnDef::new(Emails::TemplateType).string().null())
                    .col(ColumnDef::new(Emails::TemplateLanguage).string().null())
                    .col(ColumnDef::new(Emails::RenderedWith).string().null())

                    .col(ColumnDef::new(Emails::Provider).string_len(32).not_null())
                    .col(ColumnDef::new(Emails::Status).string_len(32).not_null())
                    .col(ColumnDef::new(Emails::ErrorMessage).text().null())
                    .col(ColumnDef::new(Emails::MessageId).string().null())

                    .col(ColumnDef::new(Emails::RetryCount).integer().not_null())
                    .col(ColumnDef::new(Emails::SentByFallback).boolean().not_null())

                    .col(ColumnDef::new(Emails::OpenedAt).timestamp().null())
                    .col(ColumnDef::new(Emails::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Emails::UpdatedAt).timestamp().not_null())

                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Emails::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Emails {
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