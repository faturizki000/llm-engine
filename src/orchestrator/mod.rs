pub mod executor;
pub mod metrics;
pub mod router;

pub use executor::ExecutionPlan;

/// Root orchestrator holding the execution plan for a request.
#[derive(Clone, Debug, Default)]
pub struct Orchestrator {
    offline: bool,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_offline(mut self, offline: bool) -> Self {
        self.offline = offline;
        self
    }

    pub fn plan(&self, prompt: &str) -> ExecutionPlan {
        let mut plan = ExecutionPlan::new(prompt);
        if self.offline {
            plan.provider = "local".to_string();
        }
        plan
    }
}
