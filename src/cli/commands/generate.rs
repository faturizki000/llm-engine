use crate::{LocalRuntime, RuntimeConfig};

/// `generate` command handler.
#[derive(Clone, Debug)]
pub struct GenerateCommand {
    pub prompt: String,
    pub max_tokens: usize,
}

impl GenerateCommand {
    pub fn new(prompt: String, max_tokens: usize) -> Self {
        Self { prompt, max_tokens }
    }

    pub fn run(&self) -> String {
        let runtime = LocalRuntime::new(RuntimeConfig::default());
        runtime.generate(&self.prompt, self.max_tokens)
    }
}
