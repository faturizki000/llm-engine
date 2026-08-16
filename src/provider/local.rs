use crate::provider::trait_::InferenceProvider;

/// Local provider implementation used as the hard fallback path.
#[derive(Clone, Debug, Default)]
pub struct LocalProvider;

impl InferenceProvider for LocalProvider {
    fn name(&self) -> String {
        "local".to_string()
    }

    fn is_available(&self) -> bool {
        true
    }
}
