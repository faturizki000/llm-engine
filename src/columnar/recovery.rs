/// Crash recovery stub for partially written blocks.
#[derive(Clone, Debug, Default)]
pub struct RecoveryManager {
    valid_blocks: Vec<u64>,
}

impl RecoveryManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_valid(&mut self, block_id: u64) {
        self.valid_blocks.push(block_id);
    }

    pub fn contains_valid_block(&self, block_id: u64) -> bool {
        self.valid_blocks.iter().any(|id| *id == block_id)
    }
}
