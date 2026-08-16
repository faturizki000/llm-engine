pub mod block;
pub mod compat;
pub mod index;
pub mod metadata;
pub mod store;

pub use block::KvBlock;
pub use store::KvStore;

/// Runtime key-value cache interface.
pub struct KvCache;
