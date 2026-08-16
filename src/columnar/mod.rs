pub mod index;
pub mod manifest;
pub mod read;
pub mod recovery;
pub mod schema;
pub mod storage;
pub mod write;

pub use storage::Storage;

/// Columnar storage layer for persistent, crash-safe KV persistence.
#[derive(Clone, Debug, Default)]
pub struct ColumnarStore {
    pub path: String,
    pub block_count: usize,
}

impl ColumnarStore {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            block_count: 0,
        }
    }

    pub fn add_block(&mut self) {
        self.block_count += 1;
    }
}
