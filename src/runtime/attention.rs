/// A tiny deterministic attention kernel for local inference.
#[derive(Clone, Debug, Default)]
pub struct Attention {
    pub heads: usize,
}

impl Attention {
    pub fn new(heads: usize) -> Self {
        Self { heads }
    }

    pub fn forward(&self, hidden: &[f32], context: &[f32]) -> Vec<f32> {
        let mut out = vec![0.0; hidden.len().max(context.len())];
        let scale = (hidden.len().max(context.len()) as f32).max(1.0);
        for (idx, value) in hidden.iter().enumerate() {
            let ctx = context.get(idx).copied().unwrap_or(0.0);
            out[idx] = (value + ctx) / scale;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attention_keeps_vector_size() {
        let attention = Attention::new(2);
        let out = attention.forward(&[1.0, 2.0], &[3.0, 4.0]);
        assert_eq!(out.len(), 2);
    }
}
