use sea_orm::{Database, DbConn};
use dotenvy::dotenv;
use std::env;

pub async fn establish_connection() -> DbConn {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&db_url)
    .await
    .expect("Failed to connect to database")
}