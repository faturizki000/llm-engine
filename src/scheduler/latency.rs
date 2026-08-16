/// Latency-oriented scheduling policy.
#[derive(Clone, Debug, Default)]
pub struct LatencyScheduler {
    queued: usize,
}

impl LatencyScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn queue(&mut self, jobs: usize) {
        self.queued = jobs;
    }

    pub fn queued_jobs(&self) -> usize {
        self.queued
    }
}
