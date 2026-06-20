//! [`ValidatorBootstrap`] — constructor contract for reference validators.

use crate::api::validator::types::{AlwaysValid, StdValidatorFactory};

/// Bootstrap trait for the standard reference [`Validator`](crate::Validator)
/// implementations.
pub trait ValidatorBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "validator"
    }

    /// Construct the null-object [`AlwaysValid`] validator that accepts everything.
    fn always_valid() -> AlwaysValid where Self: Sized {
        AlwaysValid
    }

    /// Return the standard validator-factory instance.
    fn std_factory() -> StdValidatorFactory where Self: Sized {
        StdValidatorFactory
    }
}
