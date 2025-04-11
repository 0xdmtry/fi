use reqwest::Client;
use serde_json::json;
use anyhow::{anyhow, Result};

pub async fn send_passcode_email(email: &str, passcode: &str) -> anyhow::Result<()> {
    let emailer_url = std::env::var("EMAILER_URL")
                        .unwrap_or_else(|_| "http://localhost:8001".to_string());

    let client = Client::new();
    let res = client.post(format!("{emailer_url}/v1/passcode"))
                .json(&json!({
                    "email": email,
                    "passcode": passcode,
                    "email_type": "passcode"
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