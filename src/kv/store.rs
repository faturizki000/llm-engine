use crate::kv::block::KvBlock;

/// In-memory KV store used to validate prefill/decode reuse semantics.
#[derive(Clone, Debug, Default)]
pub struct KvStore {
    blocks: Vec<KvBlock>,
}

impl KvStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, block: KvBlock) {
        self.blocks.push(block);
    }

    pub fn insert(&mut self, block: KvBlock) {
        self.blocks.push(block);
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn valid(&self) -> bool {
        self.blocks.iter().all(KvBlock::is_valid)
    }

    pub fn equivalent_to(&self, other: &Self) -> bool {
        self.blocks.len() == other.blocks.len()
            && self
                .blocks
                .iter()
                .zip(other.blocks.iter())
                .all(|(a, b)| a.block_id == b.block_id && a.token_count == b.token_count && a.hidden_size == b.hidden_size && a.payload == b.payload)
    }
}
