
#[derive(Debug, Clone, Copy)]
pub struct Statistics {
    pub total_count: usize,
    pub total_duration: f64,
    pub ops_per_sec: f64,
    pub min: f64,
    pub p50: f64,
    pub p75: f64,
    pub p95: f64,
    pub p99: f64,
    pub p99_9: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
}
