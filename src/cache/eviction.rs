/// Eviction policy for exact response cache entries.
#[derive(Clone, Debug, Default)]
pub struct EvictionPolicy {
    max_entries: usize,
}

impl EvictionPolicy {
    pub fn new(max_entries: usize) -> Self {
        Self { max_entries }
    }

    pub fn should_evict(&self, len: usize) -> bool {
        len > self.max_entries
    }
}
