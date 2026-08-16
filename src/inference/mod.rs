pub mod decode;
pub mod prefill;
pub mod router;
pub mod state;

/// Inference mode routing between prefill and decode.
pub enum InferencePhase {
    Prefill,
    Decode,
}
