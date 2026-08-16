/// Vocabulary manager stub.
#[derive(Clone, Debug, Default)]
pub struct Vocab {
    tokens: Vec<String>,
}

impl Vocab {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, token: impl Into<String>) {
        self.tokens.push(token.into());
    }

    pub fn id_for(&self, token: &str) -> Option<usize> {
        self.tokens.iter().position(|entry| entry == token)
    }

    pub fn token_for(&self, id: usize) -> Option<&str> {
        self.tokens.get(id).map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}
