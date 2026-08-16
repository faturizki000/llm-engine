pub mod latency;
pub mod queue;
pub mod throughput;
pub mod worker;

/// Scheduler mode used for latency- and throughput-sensitive workloads.
pub enum SchedulingMode {
    Latency,
    Throughput,
}
