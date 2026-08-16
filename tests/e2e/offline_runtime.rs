use llm_engine::{RuntimeConfig, LocalRuntime};

#[test]
fn offline_runtime_succeeds_without_network_or_api_key() {
    let runtime = LocalRuntime::new(RuntimeConfig::default());
    let result = runtime.generate("cold start", 8);
    assert!(result.starts_with("local::"));
}
