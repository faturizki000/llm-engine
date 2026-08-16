/// Basic allocator for tracking memory budgets and usage.
#[derive(Clone, Debug, Default)]
pub struct Allocator {
    capacity_bytes: usize,
    used_bytes: usize,
}

impl Allocator {
    pub fn new(capacity_bytes: usize) -> Self {
        Self {
            capacity_bytes,
            used_bytes: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if self.used_bytes + size > self.capacity_bytes {
            return None;
        }
        self.used_bytes += size;
        Some(self.used_bytes)
    }

    pub fn free(&mut self, size: usize) {
        self.used_bytes = self.used_bytes.saturating_sub(size);
    }

    pub fn usage_ratio(&self) -> f32 {
        if self.capacity_bytes == 0 {
            return 0.0;
        }
        self.used_bytes as f32 / self.capacity_bytes as f32
    }
}
