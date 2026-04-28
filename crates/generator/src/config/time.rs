use super::rand_transform::RandTransformConfig;

pub struct TimeConfig {
    pub transform: RandTransformConfig,
    pub obstacle_chance: f64,
    pub obstacle_amount: f64,
}
