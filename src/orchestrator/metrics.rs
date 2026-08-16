/// Execution metrics placeholder.
#[derive(Clone, Debug, Default)]
pub struct ExecutionMetrics {
    pub latency_ms: u64,
    pub tokens: usize,
}

impl ExecutionMetrics {
    pub fn new(latency_ms: u64, tokens: usize) -> Self {
        Self { latency_ms, tokens }
    }
}
