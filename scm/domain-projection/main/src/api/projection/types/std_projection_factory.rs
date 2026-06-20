//! [`StdProjectionFactory`] — reference implementation of [`ProjectionBootstrap`].

/// Reference implementation of [`ProjectionBootstrap`].
/// Implement this trait on any unit struct to gain the standard projection constructors.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdProjectionFactory;
