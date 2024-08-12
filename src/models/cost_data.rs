use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CostData {
    pub total_cost: f64,
    pub service_costs: HashMap<String, f64>,
}

impl CostData {
    pub fn new(total_cost: f64, service_costs: HashMap<String, f64>) -> Self {
        Self {
            total_cost,
            service_costs,
        }
    }
}

// #[derive(Debug)]
// struct ResultByTime {
//     time_period: Option<DateInterval>,
//     total: Option<HashMap<String, MetricValue>>,
//     groups: Option<Vec<Group>>,
//     estimated: bool,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Group {
//     keys: Option<Vec<String>>,
//     metrics: Option<HashMap<String, MetricValue>>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct MetricValue {
//     amount: Option<String>,
//     unit: Option<String>,
// }
