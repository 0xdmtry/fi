use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Wallets
        manager
            .create_table(
                Table::create()
                    .table(Wallets::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Wallets::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Wallets::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Wallets::WalletAddress)
                            .string_len(64)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Wallets::WalletPublicKey)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Wallets::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Wallets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // EncryptionSecrets
        manager
            .create_table(
                Table::create()
                    .table(EncryptionSecrets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EncryptionSecrets::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(EncryptionSecrets::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(EncryptionSecrets::EncryptedSecret)
                            .binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EncryptionSecrets::Salt).binary().not_null())
                    .col(
                        ColumnDef::new(EncryptionSecrets::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EncryptionSecrets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // WalletShares
        manager
            .create_table(
                Table::create()
                    .table(WalletShares::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WalletShares::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WalletShares::WalletId).uuid().not_null())
                    .col(
                        ColumnDef::new(WalletShares::EncryptedUserShare)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WalletShares::ServerShare)
                            .binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WalletShares::EncryptionSecretId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WalletShares::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WalletShares::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-walletshare-wallet_id")
                            .from(WalletShares::Table, WalletShares::WalletId)
                            .to(Wallets::Table, Wallets::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-walletshare-encryption_secret_id")
                            .from(WalletShares::Table, WalletShares::EncryptionSecretId)
                            .to(EncryptionSecrets::Table, EncryptionSecrets::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Indices
        manager
            .create_index(
                Index::create()
                    .name("idx_wallets_user_id")
                    .table(Wallets::Table)
                    .col(Wallets::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_encryption_secrets_user_id")
                    .table(EncryptionSecrets::Table)
                    .col(EncryptionSecrets::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_wallet_shares_wallet_id")
                    .table(WalletShares::Table)
                    .col(WalletShares::WalletId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WalletShares::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EncryptionSecrets::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Wallets::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Wallets {
    Table,
    Id,
    UserId,
    WalletAddress,
    WalletPublicKey,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum WalletShares {
    Table,
    Id,
    WalletId,
    EncryptedUserShare,
    ServerShare,
    EncryptionSecretId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum EncryptionSecrets {
    Table,
    Id,
    UserId,
    EncryptedSecret,
    Salt,
    CreatedAt,
    UpdatedAt,
}
