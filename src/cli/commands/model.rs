use crate::model::loader;

/// `model` command handler.
#[derive(Clone, Debug)]
pub struct ModelCommand {
    pub action: String,
}

impl ModelCommand {
    pub fn new(action: String) -> Self {
        Self { action }
    }

    pub fn run(&self) -> String {
        match self.action.as_str() {
            "list" => "Available models: tiny-deterministic (default)".to_string(),
            "info" => {
                let config = loader::load_tiny_model();
                format!("Model: tiny-deterministic, vocab: {}, layers: {}", config.vocab_size, config.num_layers)
            },
            _ => format!("Unknown model action: {}", self.action),
        }
    }
}
