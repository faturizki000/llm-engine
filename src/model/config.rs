/// Tiny-model configuration used by the local runtime.
#[derive(Clone, Debug, Default)]
pub struct ModelConfig {
    pub hidden_size: usize,
    pub num_layers: usize,
    pub vocab_size: usize,
    pub deterministic: bool,
}
