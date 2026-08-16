/// Token decode phase for incremental generation.
#[derive(Clone, Debug, Default)]
pub struct DecodePhase {
    pub token: usize,
}

impl DecodePhase {
    pub fn new(token: usize) -> Self {
        Self { token }
    }

    pub fn as_index(&self) -> usize {
        self.token
    }
}
