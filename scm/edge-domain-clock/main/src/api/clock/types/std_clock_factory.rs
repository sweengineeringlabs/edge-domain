//! [`StdClockFactory`] — reference implementation of [`ClockFactory`].

/// Reference implementation of [`ClockFactory`].
/// Implement this trait on any unit struct to gain the standard clock constructors.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdClockFactory;
