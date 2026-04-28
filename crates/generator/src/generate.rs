use crate::config::GeneratorConfig;
use domain::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn generate_problem(config: GeneratorConfig) -> Problem {
    // implementation
    let mut rng = StdRng::seed_from_u64(config.seed);

    let customers: Vec<Customer> = Vec::new();
    for _ in 1..config.num_depots {
        customers.push(Customer {
            amount: 0,
            service_time: 0,
            time_windows: Vec::new(),
        })
    }

    for _ in 0..config.num_customers {
        let amount: u64 = rng.gen_range(1..=config.max_amount);
        let service_time: u64 = rng.gen_range(0..=config.max_service_time);
        let mut customer: Customer = Customer {
            amount: amount,
            service_time: service_time,
            time_windows: Vec::new(),
        };
        if rng.gen_bool(config.time_window.chance) {
            let windows: Vec<(u64, u64)> = Vec::new();
            for _ in 0..rng.gen_range(1..=config.time_window.max_count) {
                let size: u64 = rng
                    .gen_range(config.time_window.min_size..=config.time_window.max_size)
                    + service_time;
                let start: u64 = rng.gen_range(0..=config.time_window.repeat_span);
                let mut overlaps: bool = false;
                for (start2, end2) in windows {
                    if (start2 <= start && start <= end2)
                        || (start2 <= start + size && start + size <= end2)
                    {
                        overlaps = true;
                        break;
                    }
                }
                if !overlaps {
                    windows.push((start, start + size));
                }
            }

            for (start, end) in windows {
                customer.time_windows.push(TimeWindow {
                    start: start,
                    end: end,
                });
                if config.time_window.repeats {
                    for i in 1..=5 {
                        customer.time_windows.push(TimeWindow {
                            start: start + i * config.time_window.repeat_span,
                            end: end + i * config.time_window.repeat_span,
                        });
                    }
                }
            }
        }
        customers.push(customer);
    }
    let n = config.num_customers + config.num_depots;

    let edge_costs: EdgeMatrix = EdgeMatrix::new(n);
    let edge_times: EdgeMatrix = EdgeMatrix::new(n);

    // TODO
    match config.cost_type {
        CostType::Random(min, max) => {}
        CostType::ProportionalToTime(transform) => {}
    }

    let out = Problem {
        customers: customers,
    };

    out
}
