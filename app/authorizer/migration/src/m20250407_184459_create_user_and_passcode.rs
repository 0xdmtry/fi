use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Users::Email).string_len(254).not_null().unique_key())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Users::DeletedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Passcodes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Passcodes::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Passcodes::UserId).uuid().not_null())
                    .col(ColumnDef::new(Passcodes::Code).string_len(16).not_null())
                    .col(ColumnDef::new(Passcodes::AttemptCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Passcodes::ResendCount).integer().not_null().default(0))
                    .col(ColumnDef::new(Passcodes::Used).boolean().not_null())
                    .col(ColumnDef::new(Passcodes::ExpiredAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Passcodes::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Passcodes::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-passcode-user_id")
                            .from(Passcodes::Table, Passcodes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        
        manager
            .create_index(
                Index::create()
                    .name("idx_passcode_user_id")
                    .table(Passcodes::Table)
                    .col(Passcodes::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_passcode_lookup")
                    .table(Passcodes::Table)
                    .col(Passcodes::Code)
                    .col(Passcodes::UserId)
                    .col(Passcodes::Used)
                    .col(Passcodes::ExpiredAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_passcode_attempt_count")
                    .table(Passcodes::Table)
                    .col(Passcodes::AttemptCount)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_passcode_resend_count")
                    .table(Passcodes::Table)
                    .col(Passcodes::ResendCount)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Passcodes::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Email,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum Passcodes {
    Table,
    Id,
    UserId,
    Code,
    AttemptCount,
    ResendCount,
    Used,
    ExpiredAt,
    CreatedAt,
    UpdatedAt,
}