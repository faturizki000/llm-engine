/// Request routing choices.
#[derive(Clone, Debug, Default)]
pub struct Router;

impl Router {
    pub fn route(&self, prompt: &str) -> &'static str {
        if prompt.trim().is_empty() { "local" } else { "local" }
    }
}
