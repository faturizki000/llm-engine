/// Sampling algorithm stub.
#[derive(Clone, Debug, Default)]
pub struct SamplingAlgorithms;

impl SamplingAlgorithms {
    /// Greedy sampling: select max probability token.
    pub fn greedy(logits: &[f32]) -> usize {
        logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    /// Top-k sampling: select from k highest probability tokens.
    pub fn top_k(logits: &[f32], k: usize) -> usize {
        let mut indices: Vec<usize> = (0..logits.len()).collect();
        indices.sort_by(|&a, &b| logits[b].partial_cmp(&logits[a]).unwrap_or(std::cmp::Ordering::Equal));
        let selected = indices[0..k.min(logits.len())].to_vec();
        *selected.first().unwrap_or(&0)
    }

    /// Top-p (nucleus) sampling: select from smallest set with cumulative prob >= p.
    pub fn top_p(logits: &[f32], p: f32) -> usize {
        let mut indexed: Vec<(usize, f32)> = logits.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        let total: f32 = indexed.iter().map(|(_, v)| v).sum();
        let mut cumsum = 0.0;
        for &(idx, val) in &indexed {
            cumsum += val / total;
            if cumsum >= p {
                return idx;
            }
        }
        indexed.first().map(|(i, _)| *i).unwrap_or(0)
    }

    /// Deterministic sampling (same as greedy).
    pub fn deterministic(logits: &[f32]) -> usize {
        Self::greedy(logits)
    }
}
