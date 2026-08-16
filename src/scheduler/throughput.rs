/// Throughput-oriented scheduling policy.
#[derive(Clone, Debug, Default)]
pub struct ThroughputScheduler {
    batch_size: usize,
}

impl ThroughputScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn batch(&mut self, size: usize) {
        self.batch_size = size;
    }

    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
}
