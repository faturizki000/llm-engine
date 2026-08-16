/// Compatibility helper for versioned kv blocks.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompatibilityInfo {
    pub version: u32,
    pub compatible: bool,
}

impl CompatibilityInfo {
    pub fn new(version: u32, compatible: bool) -> Self {
        Self { version, compatible }
    }
}
