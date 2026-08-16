/// Route list for potential HTTP endpoints.
#[derive(Clone, Debug, Default)]
pub struct Routes {
    pub endpoints: Vec<String>,
}

impl Routes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, endpoint: &str) {
        self.endpoints.push(endpoint.to_string());
    }
}
