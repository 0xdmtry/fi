use sea_orm::{Database, DbConn};
use std::time::Duration;
use tokio::time::sleep;
use crate::config::AppConfig;

pub async fn establish_connection(config: &AppConfig) -> DbConn {
    for attempt in 1..=config.db_conn_max_attempts {
        match Database::connect(&config.database_url).await {
            Ok(conn) => return conn,
            Err(e) => {
                eprintln!("🔁 Attempt {}/{} failed to connect to DB: {}", attempt, config.db_conn_max_attempts, e);
                if attempt < config.db_conn_max_attempts {
                    sleep(Duration::from_secs(config.db_conn_retry_delay_seconds)).await;
                }
            }
        }
    }

    panic!("❌ Could not connect to DB after {} attempts", config.db_conn_max_attempts);
}