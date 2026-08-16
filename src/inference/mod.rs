pub mod decode;
pub mod prefill;
pub mod router;
pub mod state;

/// Inference mode routing between prefill and decode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferencePhase {
    Prefill,
    Decode,
}

impl InferencePhase {
    pub fn next(&self) -> Self {
        match self {
            Self::Prefill => Self::Decode,
            Self::Decode => Self::Decode,
        }
    }
}
