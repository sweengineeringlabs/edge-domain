//! [`ValidationError`] — configuration validation failure.

/// Configuration validation failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(
    /// Human-readable description of the validation failure.
    pub String,
);
