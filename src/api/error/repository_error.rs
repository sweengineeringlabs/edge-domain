//! Error type for [`Repository`](super::repository::Repository) operations.

/// Error produced by repository data-access operations.
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    /// The requested entity was not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// A constraint was violated (e.g. unique key, foreign key).
    #[error("conflict: {0}")]
    Conflict(String),
    /// The underlying store is unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// An unexpected internal error occurred.
    #[error("internal: {0}")]
    Internal(String),
}
