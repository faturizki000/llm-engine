pub mod block;
pub mod compat;
pub mod index;
pub mod metadata;
pub mod store;

pub use block::KvBlock;
pub use store::KvStore;

/// Runtime key-value cache interface.
#[derive(Clone, Debug, Default)]
pub struct KvCache {
    entries: std::collections::HashMap<String, String>,
}

impl KvCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
