/// Deterministic RNG stub.
#[derive(Clone, Debug, Default)]
pub struct DeterministicRng {
    seed: u64,
}

impl DeterministicRng {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed
    }
}
