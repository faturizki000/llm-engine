pub mod algorithms;
pub mod distribution;
pub mod rng;

/// Sampling strategy used for generated tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SamplingMode {
    Greedy,
    Deterministic,
}
