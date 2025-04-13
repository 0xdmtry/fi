use sea_orm::{Database, DbConn};
use dotenvy::dotenv;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

const MAX_RETRIES: u8 = 10;
const RETRY_DELAY_SECS: u64 = 2;

pub async fn establish_connection() -> DbConn {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    for attempt in 1..=MAX_RETRIES {
        match Database::connect(&db_url).await {
            Ok(conn) => return conn,
            Err(e) => {
                eprintln!("🔁 Attempt {}/{} failed to connect to DB: {}", attempt, MAX_RETRIES, e);
                if attempt < MAX_RETRIES {
                    sleep(Duration::from_secs(RETRY_DELAY_SECS)).await;
                }
            }
        }
    }

    panic!("❌ Could not connect to DB after {} attempts", MAX_RETRIES);
}