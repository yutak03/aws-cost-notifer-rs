use anyhow::{Ok, Result};

use crate::models::{cost_data::CostData, cost_summary::CostSummary};

pub fn analyze_cost_data(data: &CostData) -> Result<CostSummary> {
    let total_cost = data.total_cost;
    let mut service_breakdown: Vec<(String, f64)> = data
        .service_costs
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    service_breakdown.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    Ok(CostSummary::new(total_cost, service_breakdown))
}

pub fn format_slack_message(summary: &CostSummary) -> String {
    let breakdown = summary
        .service_breakdown
        .iter()
        .map(|(service, cost)| format!("- {}: ${:.2}", service, cost))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "【今月のAWS請求金額】\n合計額: ${:.2}\nサービスごと請求金額:\n\n{}",
        summary.total_cost, breakdown
    )
}
