//! [`StepNameResponse`] — wraps a step's human-readable name.

/// Response carrying a step's name (for logging, debugging, observability).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepNameResponse {
    /// The step's human-readable name.
    pub name: String,
}
