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

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn valid(&self) -> bool {
        self.blocks.iter().all(KvBlock::is_valid)
    }
}
