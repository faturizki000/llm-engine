/// Selective read index placeholder.
#[derive(Clone, Debug, Default)]
pub struct ColumnIndex {
    pages: std::collections::HashMap<u64, usize>,
}

impl ColumnIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, block_id: u64, offset: usize) {
        self.pages.insert(block_id, offset);
    }

    pub fn offset_for(&self, block_id: u64) -> Option<usize> {
        self.pages.get(&block_id).copied()
    }
}
