/// Write protocol for traceable block writes.
#[derive(Clone, Debug, Default)]
pub struct WriteProtocol {
    pub writes: Vec<u64>,
}

impl WriteProtocol {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, block_id: u64) {
        self.writes.push(block_id);
    }
}
