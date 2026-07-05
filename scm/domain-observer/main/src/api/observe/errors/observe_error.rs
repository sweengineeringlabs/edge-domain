//! `ObserveError` — errors produced by the observability factory.

/// Error produced when an observability backend cannot be initialised.
#[derive(Debug, PartialEq, Eq)]
pub enum ObserveError {
    /// The requested backend is not available in the current environment.
    BackendUnavailable(String),
    /// The factory was called before the backend was initialised.
    NotInitialised,
}
