/// Index metadata helper for block lookup.
#[derive(Clone, Debug, Default)]
pub struct BlockIndex {
    index: std::collections::HashMap<u64, usize>,
}

impl BlockIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, block_id: u64, position: usize) {
        self.index.insert(block_id, position);
    }

    pub fn get(&self, block_id: &u64) -> Option<&usize> {
        self.index.get(block_id)
    }
}
