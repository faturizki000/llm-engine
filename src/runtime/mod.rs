pub mod attention;
pub mod embedding;
pub mod ffn;
pub mod lm_head;
pub mod transformer;

use crate::config::RuntimeConfig;
use crate::runtime::transformer::Transformer;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Local runtime for the tiny deterministic model.
#[derive(Clone, Debug)]
pub struct LocalRuntime {
    cfg: RuntimeConfig,
}

impl Default for LocalRuntime {
    fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
}

impl LocalRuntime {
    pub fn new(cfg: RuntimeConfig) -> Self {
        Self { cfg }
    }

    /// Produces deterministic output from a prompt without any external API dependency.
    pub fn generate(&self, prompt: &str, max_tokens: usize) -> String {
        let digest = {
            let mut hasher = DefaultHasher::new();
            prompt.hash(&mut hasher);
            hasher.finish()
        };
        let normalized = prompt.trim();
        let transformer = Transformer::new(8, 32);
        let score = transformer.next_token_score(normalized);
        let token_count = max_tokens.max(1);
        let mut out = format!("local::{normalized}::{}::{score:.6}", digest % 10_000);
        for i in 0..token_count {
            out.push_str(&format!("t{i}"));
        }
        out
    }

    pub fn cfg(&self) -> &RuntimeConfig {
        &self.cfg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_runtime_generates_deterministic_output() {
        let runtime = LocalRuntime::default();
        let out1 = runtime.generate("hello world", 4);
        let out2 = runtime.generate("hello world", 4);
        assert_eq!(out1, out2);
    }
}
