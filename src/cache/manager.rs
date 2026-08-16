use std::collections::HashMap;

/// Cache lifecycle manager for exact and KV reuse policies.
#[derive(Clone, Debug, Default)]
pub struct CacheManager {
    exact: HashMap<String, String>,
    kv_hits: usize,
}

impl CacheManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_exact(&mut self, key: String, value: String) {
        self.exact.insert(key, value);
    }

    pub fn get_exact(&self, key: &str) -> Option<&String> {
        self.exact.get(key)
    }

    pub fn record_kv_hit(&mut self) {
        self.kv_hits += 1;
    }

    pub fn kv_hits(&self) -> usize {
        self.kv_hits
    }
}
