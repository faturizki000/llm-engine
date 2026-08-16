use crate::scheduler::SchedulingMode;

/// `benchmark` command handler.
#[derive(Clone, Debug)]
pub struct BenchmarkCommand {
    pub mode: String,
}

impl BenchmarkCommand {
    pub fn new(mode: String) -> Self {
        Self { mode }
    }

    pub fn run(&self) -> String {
        let sched_mode = match self.mode.as_str() {
            "throughput" => SchedulingMode::Throughput,
            _ => SchedulingMode::Latency,
        };
        format!("Benchmark mode: {}, offline: true", sched_mode.as_str())
    }
}
