use std::collections::HashMap;
use anyhow::{
    Context,
    Ok,
    Result
};
use aws_config::BehaviorVersion;
use aws_config::Region;
use aws_sdk_costexplorer::Client as CostExplorerClient;
use aws_sdk_costexplorer::operation::get_cost_and_usage::GetCostAndUsageOutput;
use aws_sdk_costexplorer::types::{
    Granularity, 
    GroupDefinition, 
    GroupDefinitionType, 
    DateInterval
};

use crate::config::Config;
use crate::models::cost_data::CostData;
use crate::utils::billing_period::BillingPeriod;

pub async fn create_client(config: &Config) -> Result<CostExplorerClient> {
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(config.aws_region.clone()))
        .load()
        .await;

    Ok(CostExplorerClient::new(&aws_config))
}

pub async fn fetch_cost(client: &CostExplorerClient) -> Result<CostData> {
    let billing_period = BillingPeriod::get();
    let time_period = DateInterval::builder()
        .start(billing_period.start_date.to_string())
        .end(billing_period.end_date.to_string())
        .build()
        .expect("Failed to build DateInterval struct");

    let group = GroupDefinition::builder()
        .r#type(GroupDefinitionType::Dimension)
        .key("SERVICE")
        .build();

    let resp = client
        .get_cost_and_usage()
        .time_period(time_period)
        .granularity(Granularity::Monthly)
        .metrics("UnblendedCost")
        .group_by(group)
        .send()
        .await
        .context("Failed to fetch AWS cost data")?;

    parse_cost_explorer_response(&resp)
}

fn parse_cost_explorer_response(resp: &GetCostAndUsageOutput) -> Result<CostData> {
    let result = resp
        .clone()
        .results_by_time
        .and_then(|r| r.first().cloned())
        .context("No results found")?;

    let groups = result.groups.expect("No groups found");
    let service_costs: HashMap<String, f64> = groups
        .iter()
        .filter_map(|group| {
            let service = group
                .keys
                .as_ref()
                .and_then(|keys| keys.first())
                .map(|s| s.to_string())?;
            let amount = group
                .metrics
                .as_ref()
                .and_then(|metrics| metrics.get("UnblendedCost"))
                .and_then(|metric| metric.amount())
                .and_then(|amount| amount.parse::<f64>().ok())?;
            Some((service, amount))
        })
        .collect();
    let total_cost = service_costs.values().sum();

    Ok(CostData::new(total_cost, service_costs))
}
