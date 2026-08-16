/// Manifest metadata for on-disk storage.
#[derive(Clone, Debug, Default)]
pub struct Manifest {
    pub version: u32,
    pub block_count: usize,
}

impl Manifest {
    pub fn new(version: u32, block_count: usize) -> Self {
        Self { version, block_count }
    }
}
