use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generates a deterministic hash for prompt-based cache keys.
pub fn hash_prompt(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

/// Normalizes a path for local cache and model files.
pub fn normalize_path(path: &str) -> String {
    path.trim().replace('\\', "/")
}

/// Returns a monotonic wall-clock timestamp in nanoseconds.
pub fn now_nanos() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_prompt_is_deterministic() {
        assert_eq!(hash_prompt("hello"), hash_prompt("hello"));
    }
}
