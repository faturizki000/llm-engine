pub mod exact;
pub mod eviction;
pub mod manager;
pub mod serialization;

pub use exact::ExactResponseCache;

/// Response cache root for semantic string deduplication.
pub struct ResponseCache;
