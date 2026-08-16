use crate::inference::InferencePhase;

/// Router for phase selection.
#[derive(Clone, Debug, Default)]
pub struct InferenceRouter;

impl InferenceRouter {
    pub fn select(&self, cached_prefix_len: usize) -> InferencePhase {
        if cached_prefix_len > 0 {
            InferencePhase::Decode
        } else {
            InferencePhase::Prefill
        }
    }
}
