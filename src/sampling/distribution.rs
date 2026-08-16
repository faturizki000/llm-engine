/// Probability distribution stub.
#[derive(Clone, Debug, Default)]
pub struct Distribution {
    pub weights: Vec<f32>,
    pub normalized: Vec<f32>,
}

impl Distribution {
    pub fn from_weights(weights: Vec<f32>) -> Self {
        let normalized = Self::normalize_weights(&weights);
        Self { weights, normalized }
    }

    pub fn from_logits(logits: &[f32]) -> Self {
        let weights = Self::logits_to_probs(logits);
        let normalized = Self::normalize_weights(&weights);
        Self { weights, normalized }
    }

    pub fn normalize(&self) -> Vec<f32> {
        self.normalized.clone()
    }

    pub fn sample(&self) -> usize {
        let mut cumsum = 0.0;
        for (i, &prob) in self.normalized.iter().enumerate() {
            cumsum += prob;
            if cumsum >= 0.5 {
                return i;
            }
        }
        0
    }

    fn logits_to_probs(logits: &[f32]) -> Vec<f32> {
        let max_logit = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        logits.iter().map(|&l| (l - max_logit).exp()).collect()
    }

    fn normalize_weights(weights: &[f32]) -> Vec<f32> {
        let sum: f32 = weights.iter().sum();
        if sum <= 0.0 {
            return vec![0.0; weights.len()];
        }
        weights.iter().map(|w| w / sum).collect()
    }
}
