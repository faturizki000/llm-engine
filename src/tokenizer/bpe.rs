/// BPE tokenizer placeholder.
#[derive(Clone, Debug, Default)]
pub struct BpeTokenizer {
    vocab: Vec<String>,
}

impl BpeTokenizer {
    pub fn new() -> Self {
        Self {
            vocab: vec!["<bos>".to_string(), "<eos>".to_string(), "<pad>".to_string()],
        }
    }

    pub fn tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|token| token.to_string())
            .collect()
    }

    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }
}
