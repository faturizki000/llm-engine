/// Kernel family selection for SIMD operations.
#[derive(Clone, Copy, Debug, Default)]
pub struct KernelSet;

pub fn dot_kernel(lhs: &[f32], rhs: &[f32]) -> f32 {
    lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).sum()
}

pub fn matmul_kernel(matrix: &[f32], vector: &[f32]) -> Vec<f32> {
    let cols = vector.len();
    let rows = matrix.len() / cols.max(1);
    let mut out = vec![0.0; rows];
    for row in 0..rows {
        let start = row * cols;
        let end = start + cols;
        out[row] = matrix[start..end].iter().zip(vector.iter()).map(|(a, b)| a * b).sum();
    }
    out
}

pub fn normalize_kernel(values: &[f32], eps: f32) -> Vec<f32> {
    let mean_sq = values.iter().map(|v| v * v).sum::<f32>() / values.len().max(1) as f32;
    let inv = 1.0 / (mean_sq + eps).sqrt();
    values.iter().map(|v| v * inv).collect()
}

pub fn rope_kernel(values: &[f32], offset: usize) -> Vec<f32> {
    values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let phase = (offset + i) as f32 * 0.1;
            let factor = phase.cos() + phase.sin();
            v * factor
        })
        .collect()
}

pub fn softmax_kernel(values: &[f32]) -> Vec<f32> {
    if values.is_empty() {
        return vec![];
    }
    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = values.iter().map(|v| (v - max).exp()).collect();
    let sum = exps.iter().sum::<f32>();
    exps.into_iter().map(|v| v / sum).collect()
}
