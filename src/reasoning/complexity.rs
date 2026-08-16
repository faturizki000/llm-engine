/// Estimates task complexity from prompt length and token budget.
#[derive(Clone, Debug, Default)]
pub struct ComplexityClassifier {
    pub threshold: usize,
}

impl ComplexityClassifier {
    pub fn new(threshold: usize) -> Self {
        Self { threshold }
    }

    pub fn classify(&self, prompt: &str, max_tokens: usize) -> &'static str {
        let score = prompt.len() + max_tokens;
        if score > self.threshold {
            "high"
        } else {
            "low"
        }
    }
}
