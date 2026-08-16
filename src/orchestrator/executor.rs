use crate::provider::{local::LocalProvider, trait_::InferenceProvider};

/// Planned execution for a request, always falling back to the local provider.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionPlan {
    pub prompt: String,
    pub provider: String,
    pub uses_cache: bool,
    pub cache_key: String,
}

impl ExecutionPlan {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            provider: LocalProvider::default().name(),
            uses_cache: false,
            cache_key: format!("local:{}", prompt.trim()),
        }
    }

    pub fn with_cache(mut self, hit: bool) -> Self {
        self.uses_cache = hit;
        if hit {
            self.provider = "local".to_string();
        }
        self
    }

    pub fn local_fallback(prompt: &str) -> Self {
        Self::new(prompt).with_cache(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_plan_defaults_to_local_provider() {
        let plan = ExecutionPlan::new("hi");
        assert_eq!(plan.provider, "local");
    }

    #[test]
    fn cache_hit_marks_plan_as_cached() {
        let plan = ExecutionPlan::new("hello").with_cache(true);
        assert!(plan.uses_cache);
    }
}
