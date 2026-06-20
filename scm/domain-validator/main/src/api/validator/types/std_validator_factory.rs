//! [`StdValidatorFactory`] — reference implementation of [`ValidatorBootstrap`].

/// Reference implementation of [`ValidatorBootstrap`].
/// Implement this trait on any unit struct to gain the standard validator constructors.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdValidatorFactory;
