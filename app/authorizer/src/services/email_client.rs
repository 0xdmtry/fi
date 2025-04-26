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

    check_response(res).await
}

pub async fn send_failed_passcode_email(config: &AppConfig, email: &str) -> Result<()> {
    let client = Client::new();
    let res = client
        .post(format!("{}/v1/passcode/failed", config.emailer_url))
        .json(&json!({
            "email": email,
        }))
        .send()
        .await?;

    check_response(res).await
}

pub async fn send_success_passcode_email(config: &AppConfig, email: &str) -> Result<()> {
    let client = Client::new();
    let res = client
        .post(format!("{}/v1/passcode/success", config.emailer_url))
        .json(&json!({
            "email": email,
        }))
        .send()
        .await?;

    check_response(res).await
}

async fn check_response(res: reqwest::Response) -> Result<()> {
    if res.status().is_success() {
        Ok(())
    } else {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        Err(anyhow!("Emailer failed ({status}: {body})"))
    }
}
