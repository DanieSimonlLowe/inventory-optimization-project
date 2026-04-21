pub struct Vehicle {
    pub capacity: u64,
    pub depot_index: usize,
    // Need to start and end on depot
}

pub struct TimeWindow {
    pub start: u64,
    pub end: u64,
}

pub struct Customer {
    pub amount: u64,
    pub service_time: u64,
    pub time_windows: Vec<TimeWindow>,
    // Multiple time windos so can simulatite multiple days.
}

pub struct Problem {
    pub edge_costs: Vec<Vec<u64>>,
    pub edge_times: Vec<Vec<u64>>,
    pub customers: Vec<Customer>,
    pub vehicles: Vec<Vehicle>,
}

pub struct Route {
    pub nodes: Vec<usize>,
    pub arrival_times: Vec<u64>, // aligns with nodes
}

pub struct Solution {
    pub routes: Vec<Route>,
    pub cost: u64,
}

pub trait Solver {
    fn solve(&self, problem: &Problem) -> Solution;
    fn name(&self) -> &'static str;
}
