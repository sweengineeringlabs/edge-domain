//! [`StdProjectionFactory`] — reference implementation of [`ProjectionFactory`].

/// Reference implementation of [`ProjectionFactory`].
/// Implement this trait on any unit struct to gain the standard projection constructors.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdProjectionFactory;
