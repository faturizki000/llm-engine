use crate::runtime::attention::Attention;
use crate::runtime::embedding::Embedding;
use crate::runtime::ffn::FeedForwardNetwork;
use crate::runtime::lm_head::LanguageModelHead;

/// Tiny deterministic transformer used for local inference in tests and CI.
#[derive(Clone, Debug)]
pub struct Transformer {
    embedding: Embedding,
    attention: Attention,
    ffn: FeedForwardNetwork,
    lm_head: LanguageModelHead,
}

impl Transformer {
    pub fn new(hidden_dim: usize, vocab_size: usize) -> Self {
        Self {
            embedding: Embedding::new(hidden_dim),
            attention: Attention::new(2),
            ffn: FeedForwardNetwork::new(hidden_dim),
            lm_head: LanguageModelHead::new(vocab_size),
        }
    }

    pub fn forward(&self, prompt: &str) -> Vec<f32> {
        let embedded = self.embedding.embed(prompt);
        let attended = self.attention.forward(&embedded, &embedded);
        let ffn_out = self.ffn.forward(&attended);
        ffn_out
    }

    pub fn next_token_score(&self, prompt: &str) -> f32 {
        let hidden = self.forward(prompt);
        self.lm_head.score(&hidden)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transformer_generates_deterministic_scores() {
        let transformer = Transformer::new(8, 32);
        let score1 = transformer.next_token_score("hello");
        let score2 = transformer.next_token_score("hello");
        assert!((score1 - score2).abs() < 1e-6);
    }
}
