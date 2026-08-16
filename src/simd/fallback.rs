/// Scalar reference kernel implementations.
pub fn scalar_dot(lhs: &[f32], rhs: &[f32]) -> f32 {
    lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).sum()
}

pub fn scalar_rmsnorm(values: &[f32], eps: f32) -> Vec<f32> {
    let mean_sq = values.iter().map(|v| v * v).sum::<f32>() / values.len() as f32;
    let inv = 1.0 / (mean_sq + eps).sqrt();
    values.iter().map(|v| v * inv).collect()
}
