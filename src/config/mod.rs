use serde::{Deserialize, Serialize};

/// Runtime configuration for local inference and persistence.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// If true, disable any network or external provider access.
    pub offline: bool,
    /// Directory used for cache persistence and crash-safe metadata.
    pub cache_dir: String,
    /// Local model path for the deterministic tiny model used in tests.
    pub model_path: String,
    /// Maximum tokens to generate.
    pub max_tokens: usize,
    /// Deterministic generation for repeatable outputs.
    pub deterministic: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            offline: true,
            cache_dir: "./cache".to_string(),
            model_path: "./models/tiny-model".to_string(),
            max_tokens: 32,
            deterministic: true,
        }
    }
}
