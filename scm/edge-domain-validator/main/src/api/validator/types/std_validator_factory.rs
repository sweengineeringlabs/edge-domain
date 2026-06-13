//! [`StdValidatorFactory`] — reference implementation of [`ValidatorFactory`].

/// Reference implementation of [`ValidatorFactory`].
/// Implement this trait on any unit struct to gain the standard validator constructors.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdValidatorFactory;
