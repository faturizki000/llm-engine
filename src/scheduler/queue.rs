/// Request queue placeholder.
#[derive(Clone, Debug, Default)]
pub struct RequestQueue {
    requests: Vec<String>,
}

impl RequestQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, request: String) {
        self.requests.push(request);
    }

    pub fn len(&self) -> usize {
        self.requests.len()
    }
}
