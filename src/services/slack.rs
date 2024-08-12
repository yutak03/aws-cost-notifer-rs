use anyhow::Result;
use reqwest::Client;

pub async fn send_notification(webhook_url: &str, message: &str) -> Result<()> {
    let client = Client::new();
    let payload = serde_json::json!({
        "text": message
    });

    client.post(webhook_url).json(&payload).send().await?;

    Ok(())
}
