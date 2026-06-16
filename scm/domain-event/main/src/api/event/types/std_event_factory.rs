//! [`StdEventFactory`] — the standard concrete [`EventFactory`] implementation.
//!
//! A zero-sized marker struct; all methods are provided by the [`EventFactory`]
//! trait's default implementations. The `impl` lives in `core/` per SEA layering rules.

/// The standard implementation of [`EventFactory`].
///
/// All factory methods are inherited from [`EventFactory`] defaults.
/// Use associated-function syntax directly: `StdEventFactory::noop_bus()`.
pub struct StdEventFactory;
