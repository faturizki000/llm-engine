/// Metadata for persisted kv blocks.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BlockMetadata {
    pub block_id: u64,
    pub version: u32,
    pub token_count: usize,
}
