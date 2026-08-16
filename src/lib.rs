pub mod cache;
pub mod cli;
pub mod columnar;
pub mod config;
pub mod error;
pub mod inference;
pub mod kv;
pub mod memory;
pub mod model;
pub mod orchestrator;
pub mod provider;
pub mod reasoning;
pub mod runtime;
pub mod sampling;
pub mod scheduler;
pub mod server;
pub mod simd;
pub mod tensor;
pub mod tokenizer;
pub mod tracing;
pub mod utils;

pub use cache::ExactResponseCache;
pub use config::RuntimeConfig;
pub use orchestrator::ExecutionPlan;
pub use provider::{InferenceProvider, LocalProvider};
pub use runtime::LocalRuntime;
pub use simd::{detect_simd, dot_product, rope_apply, rmsnorm, softmax, SimdLevel};

/// Run the CLI entry point for the single binary.
pub fn run() {
    cli::run();
}
