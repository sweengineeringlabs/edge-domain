//! Validator service marker — re-exports the Validator trait.

pub use crate::api::Validator;

/// Service name constant for the validator port.
pub const VALIDATOR_SVC: &str = "validator";
