/// Data schema definition.
#[derive(Clone, Debug, Default)]
pub struct Schema {
    pub version: u32,
    pub field_names: Vec<String>,
}

impl Schema {
    pub fn new(version: u32) -> Self {
        Self { version, field_names: vec!["block_id".to_string(), "token_count".to_string()] }
    }
}
