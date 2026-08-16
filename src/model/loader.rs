use crate::model::config::ModelConfig;

/// Loads the tiny deterministic model used by CI and tests.
pub fn load_tiny_model() -> ModelConfig {
    ModelConfig {
        hidden_size: 8,
        num_layers: 2,
        vocab_size: 32,
        deterministic: true,
    }
}
