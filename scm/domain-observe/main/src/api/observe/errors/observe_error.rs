//! `ObserveError` — errors produced by the observability factory.

/// Error produced when an observability backend cannot be initialised.
#[derive(Debug, PartialEq, Eq)]
pub enum ObserveError {
    /// The requested backend is not available in the current environment.
    BackendUnavailable(String),
    /// The factory was called before the backend was initialised.
    NotInitialised,
}

impl std::fmt::Display for ObserveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendUnavailable(msg) => write!(f, "observe backend unavailable: {msg}"),
            Self::NotInitialised => write!(f, "observe backend not initialised"),
        }
    }
}

impl std::error::Error for ObserveError {}
