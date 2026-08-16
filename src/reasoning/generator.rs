/// Multi-phase generator that schedules a deterministic local response.
#[derive(Clone, Debug, Default)]
pub struct Generator {
    pub steps: Vec<String>,
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(&self, prompt: &str) -> String {
        format!("local::reason::{prompt}")
    }
}
