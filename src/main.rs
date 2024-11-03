use anyhow::Ok;
use aws_cost_notifier_rs::analyzers::cost_analyzer::{
    analyze_cost_data, format_slack_message
};

use aws_cost_notifier_rs::config::Config;
use aws_cost_notifier_rs::services::aws::{
    create_client, fetch_cost
};
use aws_cost_notifier_rs::services::slack::send_notification;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load_config()?;
    let aws_client = create_client(&config).await?;

    let cost_data = fetch_cost(&aws_client).await?;

    let summary = analyze_cost_data(&cost_data)?;
    let message = format_slack_message(&summary);

    send_notification(&config.slack_webhook_url, &message).await?;

    Ok(())
}
