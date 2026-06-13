//! [`ValidatorFactory`] — constructor contract for reference validators.

use crate::api::validator::types::{AlwaysValid, StdValidatorFactory};

/// Factory trait for the standard reference [`Validator`](crate::Validator)
/// implementations.
pub trait ValidatorFactory {
    /// Construct the null-object [`AlwaysValid`] validator that accepts everything.
    fn always_valid() -> AlwaysValid {
        AlwaysValid
    }

    /// Return the standard validator-factory instance.
    fn std_factory() -> StdValidatorFactory {
        StdValidatorFactory
    }
}
