use sea_orm::DbConn;
use sea_orm_migration::MigratorTrait;

use crate::config::AppConfig;
use migration::Migrator;

pub async fn run_migrations_if_enabled(db: &DbConn, config: &AppConfig) {
    if config.run_migrations {
        println!("🧱 Running migrations...");
        Migrator::up(db, None)
            .await
            .expect("❌ Failed to run migrations");
    } else {
        println!("⚠️  RUN_MIGRATIONS is not set to true — skipping migrations");
    }
}
