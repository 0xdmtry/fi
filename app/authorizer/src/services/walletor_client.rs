use crate::config::AppConfig;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

/// Sends a raw signup request to Walletor
pub async fn send_walletor_signup_request(
    config: &AppConfig,
    user_id: Uuid,
    passcode: &str,
) -> Result<()> {

    let client = Client::new();
    let url = format!("{}/v1/signup", config.walletor_url); // or walletor_test_url

    let res = client
        .post(&url)
        .json(&json!({
            "user_id": user_id,
            "passcode": passcode
        }))
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(anyhow!("walletor signup failed: {} - {}", status, body));
    }

    Ok(())
}
