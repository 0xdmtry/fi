pub use sea_orm_migration::prelude::*;
pub mod m20250426_205720_create_wallet_encryption_secret_wallet_share;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20250426_205720_create_wallet_encryption_secret_wallet_share::Migration,
        )]
    }
}
