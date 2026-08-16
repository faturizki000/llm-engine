pub mod collector;
pub mod event;
pub mod span;

pub use tracing::{debug, error, info, trace, warn};

/// A minimal tracing facade that keeps the runtime observability local and dependency-light.
pub fn init() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}
