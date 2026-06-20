//! [`StdClockFactory`] — reference implementation of [`ClockBootstrap`].

/// Reference implementation of [`ClockBootstrap`].
/// Implement this trait on any unit struct to gain the standard clock constructors.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdClockFactory;
