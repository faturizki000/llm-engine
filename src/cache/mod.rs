pub mod exact;
pub mod eviction;
pub mod manager;
pub mod serialization;

pub use exact::ExactResponseCache;
pub use eviction::EvictionPolicy;
pub use manager::CacheManager;
pub use serialization::CacheSerialization;

/// Response cache root for semantic string deduplication.
#[derive(Clone, Debug, Default)]
pub struct ResponseCache {
    entries: ExactResponseCache,
}

impl ResponseCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.entries.insert(key, value);
    }
}
