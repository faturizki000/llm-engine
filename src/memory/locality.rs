/// Locality-aware placement helper.
#[derive(Clone, Debug, Default)]
pub struct LocalityHint {
    pub socket: usize,
    pub core: usize,
    pub score: f32,
}

impl LocalityHint {
    pub fn new(socket: usize, core: usize, score: f32) -> Self {
        Self { socket, core, score }
    }

    pub fn prefer_local(&self) -> bool {
        self.score >= 0.5
    }
}
