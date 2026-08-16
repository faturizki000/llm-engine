pub mod executor;
pub mod metrics;
pub mod router;

pub use executor::ExecutionPlan;

/// Root orchestrator holding the execution plan for a request.
pub struct Orchestrator;
