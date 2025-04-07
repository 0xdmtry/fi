pub use sea_orm_migration::prelude::*;
pub mod m20250407_184459_create_user_and_passcode;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250407_184459_create_user_and_passcode::Migration),
        ]
    }
}
