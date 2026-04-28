use super::rand_transform::RandTransformConfig;

pub enum CostType {
    ProportionalToTime { transform: RandTransformConfig },
    Random { min: u64, max: u64 },
}
