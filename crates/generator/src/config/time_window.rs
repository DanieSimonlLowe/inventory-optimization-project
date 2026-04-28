pub struct TimeWindowConfig {
    pub chance: f64,
    pub repeat_span: u64, // how long untill repeat.
    pub max_count: usize,
    pub repeats: bool,
    pub min_size: u64,
    pub max_size: u64,
}
