//! [`EnablementResponse`] — wraps whether a validator is enabled.

/// Response carrying whether a validator is enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnablementResponse {
    /// `true` when the validator is enabled.
    pub enabled: bool,
}
