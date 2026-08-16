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
}
