/// Tiny feed-forward block used by the prototype transformer.
#[derive(Clone, Debug, Default)]
pub struct FeedForwardNetwork {
    pub hidden_dim: usize,
}

impl FeedForwardNetwork {
    pub fn new(hidden_dim: usize) -> Self {
        Self { hidden_dim }
    }

    pub fn forward(&self, hidden: &[f32]) -> Vec<f32> {
        hidden
            .iter()
            .enumerate()
            .map(|(idx, value)| value * (idx as f32 + 1.0) / self.hidden_dim as f32 + 0.1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ffn_preserves_input_shape() {
        let ffn = FeedForwardNetwork::new(8);
        let out = ffn.forward(&[1.0, 2.0, 3.0]);
        assert_eq!(out.len(), 3);
    }
}
