use serde::{Deserialize, Serialize};

/// Serialization format for exact-response cache entries.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheSerialization {
    pub entries: std::collections::HashMap<String, String>,
}

impl CacheSerialization {
    pub fn new() -> Self {
        Self { entries: std::collections::HashMap::new() }
    }

    pub fn from_entries(entries: std::collections::HashMap<String, String>) -> Self {
        Self { entries }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}
