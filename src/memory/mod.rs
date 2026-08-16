pub mod allocator;
pub mod locality;
pub mod mmap;
pub mod pool;

/// Memory layer description for allocation and locality behavior.
pub struct MemoryArena {
    pub budget_bytes: usize,
}
