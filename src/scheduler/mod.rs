pub mod latency;
pub mod queue;
pub mod throughput;
pub mod worker;

/// Scheduler mode used for latency- and throughput-sensitive workloads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchedulingMode {
    Latency,
    Throughput,
}

impl SchedulingMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Latency => "latency",
            Self::Throughput => "throughput",
        }
    }
}
