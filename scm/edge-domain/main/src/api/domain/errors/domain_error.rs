//! Domain-level error types for the assembly layer.

/// Umbrella error for the domain assembly layer.
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    /// The requested domain component is not available.
    #[error("domain component unavailable: {0}")]
    Unavailable(String),
    /// An extension hook rejected the operation.
    #[error("extension rejected: {0}")]
    ExtensionRejected(String),
}
