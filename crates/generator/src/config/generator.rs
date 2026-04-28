use super::{cost::CostType, time_window::TimeWindowConfig};

pub enum BreakConfigType {
    None,
    NZ,
}

pub struct GeneratorConfig {
    pub num_customers: usize,
    pub num_vehicles: usize,
    pub num_depots: usize,
    pub grid_size: u64,
    pub is_symmetric: bool,
    pub time_window: TimeWindowConfig,
    pub time_rand_mult: f64,
    pub cost_type: CostType,

    pub max_amount: u64,
    pub max_service_time: u64,

    pub seed: u64,

    pub break_rule_type: BreakConfigType,
}
