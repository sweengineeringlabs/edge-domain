//! `ValidatorDefault` — interface declaration for the no-op validator.

/// Public interface type representing a configuration with no field-level invariants.
///
/// The concrete [`Validator`](super::Validator) implementation lives in
/// `core/validator/validator_default`.
pub struct ValidatorDefault;
