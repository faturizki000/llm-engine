/// Special token metadata.
#[derive(Clone, Debug, Default)]
pub struct SpecialTokens {
    pub bos: String,
    pub eos: String,
    pub pad: String,
}

impl SpecialTokens {
    pub fn new() -> Self {
        Self {
            bos: "<bos>".to_string(),
            eos: "<eos>".to_string(),
            pad: "<pad>".to_string(),
        }
    }
}
