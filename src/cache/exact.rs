use std::collections::HashMap;

/// Exact response cache keyed on normalized prompt and generation config.
#[derive(Clone, Debug, Default)]
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

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
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
