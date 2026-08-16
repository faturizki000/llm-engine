/// Sequence prefill phase for prompt tokens.
#[derive(Clone, Debug, Default)]
pub struct PrefillPhase {
    pub tokens: Vec<usize>,
}

impl PrefillPhase {
    pub fn new(tokens: Vec<usize>) -> Self {
        Self { tokens }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}
