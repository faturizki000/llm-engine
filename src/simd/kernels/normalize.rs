/// RMSNorm kernel placeholder.
#[derive(Clone, Debug, Default)]
pub struct NormalizationKernel;

impl NormalizationKernel {
    pub fn rmsnorm(x: &[f32], weight: &[f32], eps: f32) -> Vec<f32> {
        let mean_sq = x.iter().map(|v| v * v).sum::<f32>() / x.len() as f32;
        let inv = 1.0 / (mean_sq + eps).sqrt();
        x.iter()
            .zip(weight.iter())
            .map(|(v, w)| v * inv * w)
            .collect()
    }

    pub fn layernorm(x: &[f32], mean: f32, var: f32, weight: &[f32], bias: &[f32], eps: f32) -> Vec<f32> {
        let inv = 1.0 / (var + eps).sqrt();
        x.iter()
            .zip(weight.iter())
            .zip(bias.iter())
            .map(|((&v, &w), &b)| (v - mean) * inv * w + b)
            .collect()
    }
}
