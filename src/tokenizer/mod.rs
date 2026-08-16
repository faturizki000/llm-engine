pub mod bpe;
pub mod special;
pub mod vocab;

/// Lightweight tokenizer abstraction.
pub struct Tokenizer {
    pub vocab_size: usize,
}
