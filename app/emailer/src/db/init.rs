use sea_orm::DbConn;
use sea_orm_migration::MigratorTrait;

use migration::Migrator;

pub async fn run_migrations_if_enabled(db: &DbConn) {
    if std::env::var("RUN_MIGRATIONS").unwrap_or_default() == "true" {
        println!("🧱 Running migrations...");
        Migrator::up(db, None)
            .await
            .expect("❌ Failed to run migrations");
    } else {
        println!("⚠️  RUN_MIGRATIONS is not set to true — skipping migrations");
    }
}