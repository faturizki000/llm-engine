/// Provider abstraction for local and optional external inference.
pub trait InferenceProvider {
    fn name(&self) -> String;
    fn is_available(&self) -> bool;
}
