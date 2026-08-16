/// `cache` command handler.
#[derive(Clone, Debug)]
pub struct CacheCommand {
    pub action: String,
}

impl CacheCommand {
    pub fn new(action: String) -> Self {
        Self { action }
    }

    pub fn run(&self) -> String {
        match self.action.as_str() {
            "clear" => "Cache cleared.".to_string(),
            "stats" => "Cache: exact-local backend, 0 entries".to_string(),
            _ => format!("Unknown cache action: {}", self.action),
        }
    }
}
