/// Block-based key/value store metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KvBlock {
    pub block_id: u64,
    pub token_count: usize,
    pub hidden_size: usize,
}

impl KvBlock {
    pub fn is_valid(&self) -> bool {
        self.token_count > 0 && self.hidden_size > 0
    }
}
