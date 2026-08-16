pub mod context;
pub mod display;
pub mod kind;

pub use context::ErrorContext;
pub use display::format_error;
pub use kind::ErrorKind;

use thiserror::Error;

/// Core application error type for all runtime failures.
#[derive(Debug, Error)]
pub enum LlmError {
    #[error("{0}")]
    Kind(ErrorKind),
    #[error("{context}: {source}")]
    Context {
        context: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub type AppResult<T> = Result<T, LlmError>;
