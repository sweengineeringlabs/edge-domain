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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_error_not_found_message_is_actionable() {
        let e = RepositoryError::NotFound("user-99".into());
        assert!(e.to_string().contains("user-99"));
    }

    #[test]
    fn test_repository_error_conflict_message_is_actionable() {
        let e = RepositoryError::Conflict("duplicate key".into());
        assert!(e.to_string().contains("duplicate key"));
    }
}
