/// Memory map placeholder for weights and kv blocks.
#[derive(Clone, Debug, Default)]
pub struct MappedRegion {
    pub path: String,
    pub size: usize,
    pub readonly: bool,
}

impl MappedRegion {
    pub fn new(path: impl Into<String>, size: usize, readonly: bool) -> Self {
        Self {
            path: path.into(),
            size,
            readonly,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
