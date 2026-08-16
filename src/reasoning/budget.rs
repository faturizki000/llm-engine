/// Compute budget allocator placeholder.
#[derive(Clone, Debug, Default)]
pub struct BudgetAllocator {
    pub max_tokens: usize,
}

impl BudgetAllocator {
    pub fn new(max_tokens: usize) -> Self {
        Self { max_tokens }
    }

    pub fn allocate(&self, complexity: &str) -> usize {
        match complexity {
            "high" => self.max_tokens.saturating_mul(2),
            _ => self.max_tokens,
        }
    }
}
