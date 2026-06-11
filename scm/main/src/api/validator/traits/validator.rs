//! Configuration validation contract.

/// Configuration validation contract.
///
/// Implemented by configuration types to validate their fields before use.
/// A permissive default implementation is provided by
/// [`crate::api::validator::types::validator_default::ValidatorDefault`].
pub trait Validator {
    /// Validate the configuration.
    ///
    /// Returns `Err` with a human-readable description when the configuration
    /// contains an invalid combination of fields.
    fn validate(&self) -> Result<(), String>;
}
