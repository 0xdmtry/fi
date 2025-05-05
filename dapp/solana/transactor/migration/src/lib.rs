pub use sea_orm_migration::prelude::*;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }
}
mod m20250505_140047_create_tx;
