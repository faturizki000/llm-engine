use std::collections::HashMap;

/// Stores columnar blocks keyed by block id.
#[derive(Clone, Debug, Default)]
pub struct Storage {
    pages: HashMap<u64, Vec<u8>>,
}

impl Storage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_block(&mut self, id: u64, payload: Vec<u8>) {
        self.pages.insert(id, payload);
    }

    pub fn read_block(&self, id: u64) -> Option<&Vec<u8>> {
        self.pages.get(&id)
    }

    pub fn contains(&self, id: u64) -> bool {
        self.pages.contains_key(&id)
    }
}
