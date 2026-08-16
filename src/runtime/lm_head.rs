/// Produces a deterministic scalar score representing the next-token preference.
#[derive(Clone, Debug, Default)]
pub struct LanguageModelHead {
    pub vocab_size: usize,
}

impl LanguageModelHead {
    pub fn new(vocab_size: usize) -> Self {
        Self { vocab_size }
    }

    pub fn score(&self, hidden: &[f32]) -> f32 {
        hidden.iter().enumerate().map(|(idx, v)| v * (idx as f32 + 1.0)).sum::<f32>() / self.vocab_size as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lm_head_returns_numeric_score() {
        let head = LanguageModelHead::new(8);
        assert!((head.score(&[1.0, 2.0, 3.0]) - 1.75).abs() < 1e-6);
    }
}
