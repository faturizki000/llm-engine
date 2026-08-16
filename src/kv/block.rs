/// Block-based key/value store metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct KvBlock {
    pub block_id: u64,
    pub token_count: usize,
    pub hidden_size: usize,
    pub payload: Vec<f32>,
}

impl KvBlock {
    pub fn new(block_id: u64, token_count: usize, hidden_size: usize, payload: Vec<f32>) -> Self {
        Self {
            block_id,
            token_count,
            hidden_size,
            payload,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.token_count > 0 && self.hidden_size > 0 && self.payload.len() >= self.token_count * self.hidden_size
    }

    pub fn has_complete_payload(&self) -> bool {
        self.is_valid() && self.payload.len() == self.token_count * self.hidden_size
    }
}
