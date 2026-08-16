/// Sampling algorithm stub.
#[derive(Clone, Debug, Default)]
pub struct SamplingAlgorithms;

impl SamplingAlgorithms {
    pub fn greedy(values: &[f32]) -> usize {
        values
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    pub fn deterministic(values: &[f32]) -> usize {
        Self::greedy(values)
    }
}
