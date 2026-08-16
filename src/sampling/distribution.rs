/// Probability distribution stub.
#[derive(Clone, Debug, Default)]
pub struct Distribution {
    pub weights: Vec<f32>,
}

impl Distribution {
    pub fn from_weights(weights: Vec<f32>) -> Self {
        Self { weights }
    }

    pub fn normalize(&self) -> Vec<f32> {
        let sum = self.weights.iter().sum::<f32>();
        if sum <= 0.0 {
            return vec![0.0; self.weights.len()];
        }
        self.weights.iter().map(|w| w / sum).collect()
    }
}
