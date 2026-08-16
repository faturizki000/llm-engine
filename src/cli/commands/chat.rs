use crate::{LocalRuntime, RuntimeConfig};

/// `chat` command handler.
#[derive(Clone, Debug)]
pub struct ChatCommand {
    pub prompt: String,
}

impl ChatCommand {
    pub fn new(prompt: String) -> Self {
        Self { prompt }
    }

    pub fn run(&self) -> String {
        let runtime = LocalRuntime::new(RuntimeConfig::default());
        let response = runtime.generate(&self.prompt, 32);
        format!("Assistant: {}", response)
    }
}
