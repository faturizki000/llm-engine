use crate::error::LlmError;

/// Formats a structured error into a readable string.
pub fn format_error(error: &LlmError) -> String {
    match error {
        LlmError::Kind(kind) => kind.to_string(),
        LlmError::Context { context, source } => format!("{context}: {source}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorKind;

    #[test]
    fn format_error_uses_kind_string() {
        let err = LlmError::Kind(ErrorKind::RuntimeFailure);
        assert!(format_error(&err).contains("runtime failure"));
    }
}
