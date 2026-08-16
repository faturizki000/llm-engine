use crate::provider::{local::LocalProvider, trait_::InferenceProvider};

/// Planned execution for a request, always falling back to the local provider.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionPlan {
    pub prompt: String,
    pub provider: String,
    pub uses_cache: bool,
}

impl ExecutionPlan {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            provider: LocalProvider::default().name(),
            uses_cache: false,
        }
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
}
