/// Deterministic character embedding used by the tiny local model.
#[derive(Clone, Debug, Default)]
pub struct Embedding {
    pub dim: usize,
}

impl Embedding {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }

    pub fn embed(&self, text: &str) -> Vec<f32> {
        let mut result = vec![0.0; self.dim];
        for (idx, ch) in text.chars().enumerate() {
            let v = (ch as u32 as f32 + 1.0) / (idx as f32 + 1.0);
            let pos = idx % self.dim;
            result[pos] += v;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedding_produces_fixed_dimensional_vector() {
        let embedding = Embedding::new(8);
        let vec = embedding.embed("hi");
        assert_eq!(vec.len(), 8);
    }
}
