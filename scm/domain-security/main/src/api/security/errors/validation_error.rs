//! [`ValidationError`] — configuration validation failure.

/// Configuration validation failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(
    /// Human-readable description of the validation failure.
    pub String,
);

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ValidationError {}
