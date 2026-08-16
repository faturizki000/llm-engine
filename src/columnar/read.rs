/// Read protocol used to access persisted blocks.
#[derive(Clone, Debug, Default)]
pub struct ReadProtocol {
    pub block_id: Option<u64>,
}

impl ReadProtocol {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_block(block_id: u64) -> Self {
        Self { block_id: Some(block_id) }
    }
}
