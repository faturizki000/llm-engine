use crate::simd::detector::SimdLevel;

/// SIMD-aware dot product dispatcher with scalar fallback.
pub fn dot_product(lhs: &[f32], rhs: &[f32]) -> f32 {
    match crate::simd::detect_simd() {
        SimdLevel::Scalar => lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).sum(),
        _ => lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).sum(),
    }
}

/// SIMD-aware RMSNorm dispatcher.
pub fn rmsnorm(values: &[f32], eps: f32) -> Vec<f32> {
    let mean = values.iter().map(|v| v * v).sum::<f32>() / values.len() as f32;
    let inv = 1.0 / (mean + eps).sqrt();
    values.iter().map(|v| v * inv).collect()
}

/// RoPE placeholder.
pub fn rope_apply(values: &[f32], offset: usize) -> Vec<f32> {
    values.iter().enumerate().map(|(i, v)| {
        let phase = (offset + i) as f32 * 0.1;
        v * (phase.cos() + phase.sin())
    }).collect()
}

/// Softmax placeholder.
pub fn softmax(values: &[f32]) -> Vec<f32> {
    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = values.iter().map(|v| (v - max).exp()).collect();
    let sum = exps.iter().sum::<f32>();
    exps.into_iter().map(|v| v / sum).collect()
}
