/// Error contextual metadata used for debugging and retries.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ErrorContext {
    pub step: String,
    pub module: String,
    pub detail: String,
}
