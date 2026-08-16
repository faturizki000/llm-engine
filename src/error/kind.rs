/// Structured classification of runtime errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidInput,
    InvalidConfig,
    ModelNotFound,
    CacheCorrupted,
    CacheMiss,
    RuntimeFailure,
    NetworkUnavailable,
    Io(String),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "invalid input"),
            Self::InvalidConfig => write!(f, "invalid config"),
            Self::ModelNotFound => write!(f, "model not found"),
            Self::CacheCorrupted => write!(f, "cache corrupted"),
            Self::CacheMiss => write!(f, "cache miss"),
            Self::RuntimeFailure => write!(f, "runtime failure"),
            Self::NetworkUnavailable => write!(f, "network unavailable"),
            Self::Io(reason) => write!(f, "io error: {reason}"),
        }
    }
}
