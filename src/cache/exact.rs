use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Exact response cache keyed on normalized prompt and generation config.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExactResponseCache {
    entries: HashMap<String, String>,
}

impl ExactResponseCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.entries.insert(key, value);
    }

    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get_or_insert(&mut self, key: String, value: String) -> String {
        if let Some(existing) = self.entries.get(&key).cloned() {
            existing
        } else {
            self.entries.insert(key.clone(), value.clone());
            value
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_supports_exact_lookup() {
        let mut cache = ExactResponseCache::new();
        cache.insert("prompt".to_string(), "answer".to_string());
        assert_eq!(cache.get("prompt"), Some(&"answer".to_string()));
    }
}
