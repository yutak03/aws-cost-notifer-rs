use anyhow::Context;
use anyhow::Result;
use std::env::var;
use dotenvy::dotenv;

pub struct Config {
    pub aws_region: String,
    pub slack_webhook_url: String,
}

impl Config {
    pub fn load_config() -> Result<Self> {
        dotenv().ok();
        let config = Self {
            aws_region: var("AWS_REGION").context("AWS_REGION must be set")?,
            slack_webhook_url: var("SLACK_WEBHOOK_URL").context("SLACK_WEBHOOK_URL must be set")?,
        };
        Ok(config)
    }
}
