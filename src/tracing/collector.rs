/// Simple collector placeholder for span/event funneling.
#[derive(Clone, Debug, Default)]
pub struct TraceCollector {
    events: Vec<String>,
}

impl TraceCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, event: impl Into<String>) {
        self.events.push(event.into());
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
