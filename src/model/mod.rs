pub mod compat;
pub mod config;
pub mod loader;
pub mod weights;

/// Model configuration file metadata.
pub struct ModelMetadata {
    pub name: String,
    pub path: String,
    pub vocab_size: usize,
}
