pub mod index;
pub mod manifest;
pub mod read;
pub mod recovery;
pub mod schema;
pub mod storage;
pub mod write;

pub use storage::Storage;

/// Columnar storage layer for persistent, crash-safe KV persistence.
pub struct ColumnarStore;
