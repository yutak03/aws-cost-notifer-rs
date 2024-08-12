#[derive(Debug, Clone)]
pub struct CostSummary {
    pub total_cost: f64,
    pub service_breakdown: Vec<(String, f64)>,
}

impl CostSummary {
    pub fn new(total_cost: f64, service_breakdown: Vec<(String, f64)>) -> Self {
        Self {
            total_cost,
            service_breakdown,
        }
    }
}
