//! Configuration validation contract.

/// Configuration validation contract.
///
/// Implemented by configuration types to validate their fields before use.
pub trait Validator {
    /// Validate the configuration.
    ///
    /// Returns `Err` with a human-readable description when the configuration
    /// contains an invalid combination of fields.
    fn validate(&self) -> Result<(), String>;
}
