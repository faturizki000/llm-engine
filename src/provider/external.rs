use crate::provider::trait_::InferenceProvider;

/// Optional external provider path.
#[derive(Clone, Debug, Default)]
pub struct ExternalProvider {
    pub api_key: Option<String>,
}

impl InferenceProvider for ExternalProvider {
    fn name(&self) -> String {
        "external".to_string()
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
}
