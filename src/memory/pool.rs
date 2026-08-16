/// Simple pool placeholder.
#[derive(Clone, Debug, Default)]
pub struct BufferPool {
    capacity: usize,
    free: Vec<Vec<u8>>,
}

impl BufferPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            free: Vec::with_capacity(capacity),
        }
    }

    pub fn acquire(&mut self, size: usize) -> Vec<u8> {
        if let Some(buf) = self.free.pop() {
            if buf.len() >= size {
                return buf;
            }
        }
        vec![0; size]
    }

    pub fn release(&mut self, mut buffer: Vec<u8>) {
        if self.free.len() < self.capacity {
            buffer.fill(0);
            self.free.push(buffer);
        }
    }
}
