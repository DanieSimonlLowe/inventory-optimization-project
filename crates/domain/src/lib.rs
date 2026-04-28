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
    pub amount: u64, // use zero for deport.
    pub service_time: u64,
    pub time_windows: Vec<TimeWindow>,
    // Multiple time windos so can simulatite multiple days.
}

pub struct BreakRule {
    pub after_work: u64,
    pub duration: u64,
    pub must_end_deport: bool,
    pub count_break_as_work: bool,
}

pub struct EdgeMatrix {
    data: Vec<u64>,
    n: usize,
}

impl EdgeMatrix {
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![0; n * n],
            n,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> u64 {
        self.data[row * self.n + col]
    }

    fn set(&mut self, row: usize, col: usize, val: u64) {
        self.data[row * self.n + col] = val;
    }
}

pub struct Problem {
    pub edge_costs: EdgeMatrix,
    pub edge_times: EdgeMatrix,
    pub customers: Vec<Customer>,
    pub vehicles: Vec<Vehicle>,
    pub break_rules: Vec<BreakRule>,
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
