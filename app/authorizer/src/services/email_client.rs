use crate::config::AppConfig;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::json;

pub async fn send_passcode_email(
    config: &AppConfig,
    email: &str,
    passcode: &str,
) -> anyhow::Result<()> {
    let client = Client::new();
    let res = client
        .post(format!("{}/v1/passcode", config.emailer_url))
        .json(&json!({
            "email": email,
            "passcode": passcode,
        }))
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(anyhow!("Emailer failed ({status}): {body}"));
    }

    Ok(())
}
