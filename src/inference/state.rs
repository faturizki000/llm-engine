/// State machine for inference flow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceState {
    Idle,
    Prefill,
    Decode,
    Done,
}

impl Default for InferenceState {
    fn default() -> Self {
        Self::Idle
    }
}
