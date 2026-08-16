/// Worker pool placeholder.
#[derive(Clone, Debug, Default)]
pub struct WorkerPool {
    workers: usize,
}

impl WorkerPool {
    pub fn new(workers: usize) -> Self {
        Self { workers }
    }

    pub fn workers(&self) -> usize {
        self.workers
    }

    pub fn is_empty(&self) -> bool {
        self.workers == 0
    }

    pub fn add_worker(&mut self) {
        self.workers += 1;
    }

    pub fn remove_worker(&mut self) {
        self.workers = self.workers.saturating_sub(1);
    }
}
