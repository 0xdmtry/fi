use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::Email).string_len(254).not_null().unique_key())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Passcode::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Passcode::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Passcode::UserId).uuid().not_null())
                    .col(ColumnDef::new(Passcode::Code).string_len(16).not_null())
                    .col(ColumnDef::new(Passcode::ExpiredAt).timestamp().not_null())
                    .col(ColumnDef::new(Passcode::Used).boolean().not_null())
                    .col(ColumnDef::new(Passcode::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Passcode::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-passcode-user_id")
                            .from(Passcode::Table, Passcode::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_passcode_code")
                    .table(Passcode::Table)
                    .col(Passcode::Code)
                    .to_owned(),
            )
            .await?;

        
        manager
            .create_index(
                Index::create()
                    .name("idx_passcode_user_id")
                    .table(Passcode::Table)
                    .col(Passcode::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_passcode_lookup")
                    .table(Passcode::Table)
                    .col(Passcode::Code)
                    .col(Passcode::UserId)
                    .col(Passcode::Used)
                    .col(Passcode::ExpiredAt)
                    .to_owned(),
            )
            .await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Passcode::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Email,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum Passcode {
    Table,
    Id,
    UserId,
    Code,
    ExpiredAt,
    Used,
    CreatedAt,
    UpdatedAt,
}