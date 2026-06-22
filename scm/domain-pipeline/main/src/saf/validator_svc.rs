//! Validator service facade — provides validation interface.

pub use crate::api::Validator;

/// Marker constant for validator service identification.
pub const VALIDATOR_SVC: &str = "validator";

/// Create a config validator strategy.
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    crate::spi::create_validator(enabled)
}
