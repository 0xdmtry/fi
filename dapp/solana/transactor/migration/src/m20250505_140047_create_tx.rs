use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tx::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tx::Signature).string().primary_key())
                    .col(ColumnDef::new(Tx::UserId).uuid().not_null())
                    .col(ColumnDef::new(Tx::WalletId).uuid().null())
                    .col(ColumnDef::new(Tx::Slot).big_unsigned().not_null())
                    .col(ColumnDef::new(Tx::BlockTime).big_integer().null())
                    .col(ColumnDef::new(Tx::Success).boolean().not_null())
                    .col(ColumnDef::new(Tx::FeePayer).string().not_null())
                    .col(ColumnDef::new(Tx::SignerAddresses).json().not_null())
                    .col(ColumnDef::new(Tx::Instructions).json().null())
                    .col(ColumnDef::new(Tx::LogMessages).json().null())
                    .col(
                        ColumnDef::new(Tx::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tx::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tx::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Tx {
    Table,
    Signature,
    UserId,
    WalletId,
    Slot,
    BlockTime,
    Success,
    FeePayer,
    SignerAddresses,
    Instructions,
    LogMessages,
    CreatedAt,
    UpdatedAt,
}
