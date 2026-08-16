/// Verifies local inference output is structurally sane and offline safe.
#[derive(Clone, Debug, Default)]
pub struct Verifier;

impl Verifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn verify(&self, output: &str, offline: bool) -> bool {
        if output.trim().is_empty() {
            return false;
        }
        if !offline && output.starts_with("local::") {
            return false;
        }
        true
    }
}
