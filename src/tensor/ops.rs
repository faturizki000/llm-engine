/// Simple scalar ops used in tests and runtime stubs.
pub fn dot(lhs: &[f32], rhs: &[f32]) -> f32 {
    lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).sum()
}

pub fn softmax(values: &[f32]) -> Vec<f32> {
    if values.is_empty() { return vec![]; }
    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = values.iter().map(|v| (v - max).exp()).collect();
    let sum: f32 = exps.iter().sum();
    exps.iter().map(|v| v / sum).collect()
}

pub fn rmsnorm(values: &[f32], eps: f32) -> Vec<f32> {
    let mean_sq = values.iter().map(|v| v * v).sum::<f32>() / values.len() as f32;
    let inv = 1.0 / (mean_sq + eps).sqrt();
    values.iter().map(|v| v * inv).collect()
}
