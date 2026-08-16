pub mod budget;
pub mod complexity;
pub mod generator;
pub mod verifier;

pub use budget::BudgetAllocator;
pub use complexity::ComplexityClassifier;
pub use generator::Generator;
pub use verifier::Verifier;

/// Reasoning configuration and high-level task classification.
#[derive(Clone, Debug, Default)]
pub struct ReasoningContext {
    pub prompt: String,
    pub max_tokens: usize,
    pub offline: bool,
}

impl ReasoningContext {
    pub fn new(prompt: &str, max_tokens: usize, offline: bool) -> Self {
        Self {
            prompt: prompt.to_string(),
            max_tokens,
            offline,
        }
    }
}
